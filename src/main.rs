pub mod resource_scanner {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use robotics_lib::interface::{Tools, robot_map, robot_view, discover_tiles, one_direction_view};
    use robotics_lib::runner::Runnable;
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use robotics_lib::utils::LibError;
    use resource_scanner_tool::map_coordinate::MapCoordinate;
    use resource_scanner_tool::tool_errors::ToolError;
    use resource_scanner_tool::tool_errors::ToolError::*;

    /// Represents different scanning patterns used in the resource scanner tool.
    ///
    /// The `Pattern` enum is used to specify the scanning behavior, and each variant
    /// includes a `usize` field indicating the range or size of the scan.
    ///
    /// # Variants
    ///
    /// - `Area(usize)`: Scans in a square area with a side length specified by the `usize` parameter.
    /// - `DirectionUp(usize)`: Scans in an upward direction with the specified distance.
    /// - `DirectionRight(usize)`: Scans in a rightward direction with the specified distance.
    /// - `DirectionLeft(usize)`: Scans in a leftward direction with the specified distance.
    /// - `DirectionDown(usize)`: Scans in a downward direction with the specified distance.
    /// - `DiagonalUpperLeft(usize)`: Scans diagonally in the upper-left direction with the specified distance.
    /// - `DiagonalUpperRight(usize)`: Scans diagonally in the upper-right direction with the specified distance.
    /// - `DiagonalLowerLeft(usize)`: Scans diagonally in the lower-left direction with the specified distance.
    /// - `DiagonalLowerRight(usize)`: Scans diagonally in the lower-right direction with the specified distance.
    /// - `StraightStar(usize)`: Scans in a star pattern in all directions with the specified distance.
    /// - `DiagonalStar(usize)`: Scans in a star pattern diagonally in all directions with the specified distance.
    ///
    /// ASCII drawing for `StraightStar(2)`:
    ///
    /// ```plaintext
    ///      *
    ///      *
    ///    **r**
    ///      *
    ///      *
    /// ```
    ///
    /// ASCII drawing for `DiagonalStar(3)`:
    ///
    /// ```plaintext
    ///  *     *
    ///   *   *
    ///    * *
    ///     r
    ///    * *
    ///   *   *
    ///  *     *
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// // Scan in a square area with a side length of 5.
    /// use resource_scanner_tool::map_coordinate::Pattern;
    /// let area_scan = Pattern::Area(5);
    ///
    /// // Scan upward with a distance of 3.
    /// let up_scan = Pattern::DirectionUp(3);
    /// ```
    pub enum Pattern {
        Area(usize),
        DirectionUp(usize),
        DirectionRight(usize),
        DirectionLeft(usize),
        DirectionDown(usize),
        DiagonalUpperLeft(usize),
        DiagonalUpperRight(usize),
        DiagonalLowerLeft(usize),
        DiagonalLowerRight(usize),
        StraightStar(usize),
        DiagonalStar(usize)
    }

    impl Pattern {
        /// Checks if the given size is valid, that is if it is 0 or negative or if it is not
        /// odd in the case of `Pattern::Area`
        /// # Returns
        /// Returns `true` if the size is valid, `false` otherwise
        fn check_size(&self) -> bool {
            return match self {
                Pattern::Area(size) if size % 2 == 0 || (*size as i32) < 3 => {
                    false
                }
                Pattern::DirectionUp(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DirectionRight(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DirectionLeft(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DirectionDown(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DiagonalUpperLeft(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DiagonalUpperRight(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DiagonalLowerLeft(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DiagonalLowerRight(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::StraightStar(size) if (*size as i32) < 1 => {
                    false
                }
                Pattern::DiagonalStar(size) if (*size as i32) < 1 => {
                    false
                }
                _ => true
            }
        }
    }

    pub struct ResourceScanner {}

    impl Tools for ResourceScanner {}

    impl ResourceScanner {
        /// The scan function scans an area around the robot for the required content according to the pattern.
        /// it returns an option containing tile coordinates and number of contents if some content is found.
        /// if no content is found in the given area it returns None
        /// if the robot didn't have enough energy an Error
        ///
        /// # Energy Cost
        ///
        pub fn scan(&mut self,
                    world: &mut World,
                    robot: &mut impl Runnable,
                    pattern: Pattern,
                    content: Content
        ) -> Result<Option<(MapCoordinate, usize)>, Box<dyn Error>> {
            // check if the given pattern size is valid
            if !pattern.check_size() {
                return Err(Box::new(InvalidSizeError));
            }
            // get coordinates of tiles to scan
            let sanitized_coordinates = ResourceScanner::get_sanitized_tiles(robot, world, &pattern);

            let sanitized_coordinates_as_slice = sanitized_coordinates.iter()
                .map(|x| x as (usize, usize))
                .collect::<Vec<_>>()
                .as_slice();

            // discover the tiles
            let tiles = discover_tiles(robot, world, &sanitized_coordinates_as_slice);

            return match tiles {
                Ok(mut hashmap) => {
                    // retain only the tiles containing the requested content
                    hashmap.retain(|key, val| val.unwrap().content == content);
                    // if the hashmap is empty, return None
                    if hashmap.is_empty() {
                        return Ok(None);
                    }
                    // create a vector containing tile coordinates and corresponding content quantity
                    let mut tile_vec: Vec<(MapCoordinate, usize)> = Vec::new();
                    for (key, val) in hashmap.iter() {
                        tile_vec.push((key as MapCoordinate, val.unwrap().content.get_value().0.unwrap()));
                    }
                    // find the tile coordinate corresponding to the max value
                    let result = tile_vec.iter().max_by_key(|x| x.1).cloned().unwrap();
                    // return the result
                    Ok(Some(result))
                },
                Err(error) => {
                    return match error {
                        LibError::NotEnoughEnergy => Err(Box::new(ToolError::NotEnoughEnergy)),
                        LibError::NoMoreDiscovery => Err(Box::new(ToolError::NoMoreDiscovery)),
                        other => Err(Box::new(ToolError::Other(format!("{:?}", other)))),
                    }
                }
            }
        }


        /// Computes and returns a vector of target coordinates based on the given pattern.
        ///
        /// # Arguments
        ///
        /// * `robot` - A mutable reference to an object implementing the `Runnable` trait.
        /// * `world` - A reference to the `World` in which the coordinates are computed.
        /// * `pattern` - A reference to the `Pattern` that defines the coordinate computation.
        ///
        /// # Returns
        ///
        /// Returns an `Option<Vec<map_coordinate>>` representing the vector of target coordinates.
        /// Returns `None` if no valid coordinates are found.
        ///
        /// # Examples
        ///
        /// ```
        /// use resource-scanner-tool::{Runnable, World, Pattern, map_coordinate, get_coordinates};
        ///
        /// // Create objects and define pattern
        /// let mut robot = create_robot();
        /// let world = create_world();
        /// let pattern = Pattern::Area(3);
        ///
        /// // Get target coordinates
        /// let coordinates = get_coordinates(&mut robot, &world, &pattern);
        /// println!("{:?}", coordinates);
        /// ```
        fn get_target_coordinates(robot: &mut impl Runnable, world: &World, pattern: &Pattern) -> Option<Vec<MapCoordinate>> {
            let mut out = Vec::new();
            let world_size = robot_map(world).unwrap().len();
            let (x_robot, y_robot) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());

            // according to the pattern, compute the corresponding tile coordinates
            match pattern {
                Pattern::Area(size) => {
                    let length = *size as i32;
                    let x_area_robot = length / 2;
                    let y_area_robot = length / 2;
                    for x in 0..length {
                        for y in 0..length {
                            // compute the tile coordinates in the world FoR (Frame of Reference) from the tile coordinates in the area FoR
                            let x_world = (x_robot as i32) + x - x_area_robot;
                            let y_world = (y_robot as i32) + y - y_area_robot;
                            // check if the coordinates are out of bound, if so omit them
                            if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                                out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                            }
                        }
                    }
                }

                Pattern::DirectionLeft(size) => {
                    let length = *size as i32;
                    let y_world = y_robot as i32;
                    for x in 0..-length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let x_world = (x_robot as i32) + x;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DirectionRight(size) => {
                    let length = *size as i32;
                    let y_world = y_robot as i32;
                    for x in 0..length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let x_world = (x_robot as i32) + x;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DirectionUp(size) => {
                    let length = *size as i32;
                    let x_world = x_robot as i32;
                    for y in 0..length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DirectionDown(size) => {
                    let length = *size as i32;
                    let x_world = x_robot as i32;
                    for y in 0..-length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalUpperLeft(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = -i;
                        let y = -i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalUpperRight(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = i;
                        let y = -i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalLowerLeft(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = -i;
                        let y = i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalLowerRight(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = i;
                        let y = i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalStar(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        for multiplier in [(1, 1), (1, -1), (-1, 1), (1, 1)] {
                            let x = multiplier.0 * i;
                            let y = multiplier.1 * i;
                            // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                            let x_world = (x_robot as i32) + x;
                            let y_world = (y_robot as i32) + y;
                            // check if the coordinates are out of bound, if so omit them
                            if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                                out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                            }
                        }
                    }
                }

                Pattern::StraightStar(size) => {
                    let length = *size as i32;

                    // horizontal arms
                    let y_world = y_robot as i32;
                    for x in -length..length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let x_world = (x_robot as i32) + x;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }

                    // vertical arms
                    let x_world = x_robot as i32;
                    for y in -length..length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32) - 1 || y_world < 0 || y_world > (world_size as i32) - 1) {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }
            }

            return if out.len() == 0 {
                None
            } else {
                Some(out)
            }
        }


        /// Returns a vector of sanitized coordinates to be scanned based on the provided pattern,
        /// excluding coordinates already known by the robot.
        ///
        /// # Arguments
        ///
        /// * `robot` - A mutable reference to an object implementing the `Runnable` trait.
        /// * `world` - A reference to the `World` in which the coordinates are scanned.
        /// * `pattern` - A reference to the `Pattern` that defines the scanning coordinates.
        ///
        /// # Returns
        ///
        /// Returns a vector of `map_coordinate` representing the sanitized coordinates.
        ///
        /// # Errors
        ///
        /// Returns an empty vector if no target coordinates are found.
        ///
        /// # Examples
        ///
        /// ```
        /// use resource-scanner-tool::{Runnable, World, Pattern, map_coordinate, get_sanitized_tiles};
        ///
        ///
        /// // Get sanitized coordinates
        /// let sanitized_coordinates = get_sanitized_tiles(&mut robot, &world, &pattern);
        /// println!("{:?}", sanitized_coordinates);
        /// ```
        fn get_sanitized_tiles(robot: &mut impl Runnable, world: &World, pattern: &Pattern) -> Vec<MapCoordinate> {
            let mut target_vector = ResourceScanner::get_target_coordinates(robot, world, pattern)?;

            for (index, coordinate) in target_vector.iter().enumerate() {
                let known_coordinates = robot_map(world).unwrap();
                if known_coordinates[coordinate.get_height()][coordinate.get_width()].is_none() {
                    target_vector.remove(index);
                }
            }
            target_vector
        }
    }
}

fn main() {

}