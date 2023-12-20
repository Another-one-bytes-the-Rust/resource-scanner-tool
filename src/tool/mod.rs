pub mod resource_scanner {
    use std::collections::HashMap;
    use crate::coordinates::map_coordinate::MapCoordinate;
    use crate::errors::tool_errors::ToolError;
    use crate::errors::tool_errors::ToolError::*;
    use robotics_lib::interface::{discover_tiles, robot_map, robot_view, Tools};
    use robotics_lib::runner::Runnable;
    use robotics_lib::utils::LibError;
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use std::error::Error;
    use std::mem;

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
    /// use resource_scanner_tool::tool::resource_scanner::Pattern;
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
        DiagonalStar(usize),
    }

    impl Pattern {
        /// Checks if the given size is valid, that is if it is 0 or negative or if it is not
        /// odd in the case of `Pattern::Area`
        /// # Returns
        /// Returns `true` if the size is valid, `false` otherwise
        fn check_size(&self) -> bool {
            return match self {
                Pattern::Area(size) if size % 2 == 0 || (*size as i32) < 3 => false,
                Pattern::DirectionUp(size) if (*size as i32) < 1 => false,
                Pattern::DirectionRight(size) if (*size as i32) < 1 => false,
                Pattern::DirectionLeft(size) if (*size as i32) < 1 => false,
                Pattern::DirectionDown(size) if (*size as i32) < 1 => false,
                Pattern::DiagonalUpperLeft(size) if (*size as i32) < 1 => false,
                Pattern::DiagonalUpperRight(size) if (*size as i32) < 1 => false,
                Pattern::DiagonalLowerLeft(size) if (*size as i32) < 1 => false,
                Pattern::DiagonalLowerRight(size) if (*size as i32) < 1 => false,
                Pattern::StraightStar(size) if (*size as i32) < 1 => false,
                Pattern::DiagonalStar(size) if (*size as i32) < 1 => false,
                _ => true,
            };
        }
    }

    pub struct ResourceScanner {}

    impl Tools for ResourceScanner {}

    impl ResourceScanner {
        /// The scan function scans an area around the robot for the required content according to the pattern.

        /// # Arguments
        ///
        /// - `world`: A mutable reference to the world where the robot operates.
        /// - `robot`: A mutable reference to the robot.
        /// - `pattern`: The pattern defining the area to be scanned.
        /// - `content`: The content to be searched for in the area.
        ///
        /// # Returns
        ///
        /// Returns a `Result` containing either:
        /// - `Some((coordinates, count))`: If content is found, where `coordinates` is the location and `count` is the number of occurrences.
        /// - `None`: If no content is found.
        /// - `Err`: If the robot doesn't have enough energy to perform the scan.
        ///
        ///
        /// # Energy Cost
        ///
        /// This tool uses the underlying interface `discover_tile` to discover tiles. Since it uses
        /// 3 energy for each discovered tile, the scan function first checks if enough energy is present
        /// to complete the task.
        /// The following are the different energy costs based on pattern and size (assuming no tiles
        /// have already been discovered):
        ///
        /// - `Area(size)`: free if size = 3, else 12 * (size - 1)
        /// - `DirectionUp(size)`: 3 * size
        /// - `DirectionRight(size)`: 3 * size
        /// - `DirectionLeft(size)`: 3 * size
        /// - `DirectionDown(size)`: 3 * size
        /// - `DiagonalUpperLeft(size)`: 3 * size
        /// - `DiagonalUpperRight(size)`: 3 * size
        /// - `DiagonalLowerLeft(size)`: 3 * size
        /// - `DiagonalLowerRight(size)`: 3 * size
        /// - `StraightStar(size)`: 12 * size
        /// - `DiagonalStar(size)`: 12 * size
        ///
        pub fn scan(
            &mut self,
            world: &mut World,
            robot: &mut impl Runnable,
            pattern: Pattern,
            content: Content,
        ) -> Result<Option<(MapCoordinate, usize)>, Box<dyn Error>> {
            // check if the given pattern size is valid
            if !pattern.check_size() {
                return Err(Box::new(InvalidSizeError));
            }
            // check whether using robot_view is more convenient
            let use_robot_view;
            match pattern {
                Pattern::Area(3) => use_robot_view = true,
                _ => use_robot_view = false
            }

            // get coordinates of tiles to scan
            let sanitized_coordinates =
                ResourceScanner::get_sanitized_tiles(robot, world, &pattern);

            let binding = sanitized_coordinates
                .iter()
                .map(|x| (*x).into())
                .collect::<Vec<_>>();

            // discover the tiles
            let tiles;

            if use_robot_view {
                // closure converting robot_view output to discover_tiles output
                let to_hashmap = |tilemap: Vec<Vec<Option<Tile>>>| ->  Result<HashMap<(usize, usize), Option<Tile>>, LibError> {
                    let mut hashmap = HashMap::new();
                    let x_robot = robot.get_coordinate().get_col();
                    let y_robot = robot.get_coordinate().get_row();
                    for (y_area, tile_vec) in tilemap.iter().enumerate() {
                        for (x_area, tile) in tile_vec.iter().enumerate() {
                            match tile {
                                Some(t) => {
                                    let x = x_robot + x_area - 1;
                                    let y = y_robot + y_area - 1;
                                    hashmap.insert((x, y),Some(t.to_owned()))
                                },
                                None => None
                            };
                        }
                    }
                    return Ok(hashmap)
                };
                tiles = to_hashmap(robot_view(robot,world))
            }
            else {
                tiles = discover_tiles(robot, world, &binding);
            }

            return match tiles {
                Ok(mut hashmap) => {
                    // retain only the tiles containing the requested content
                    hashmap.retain(|_key, val| mem::discriminant(&val.as_ref().unwrap().content) == mem::discriminant(&content));
                    // if the hashmap is empty, return None
                    if hashmap.is_empty() {
                        return Ok(None);
                    }
                    // create a vector containing tile coordinates and corresponding content quantity
                    let mut tile_vec: Vec<(MapCoordinate, usize)> = Vec::new();
                    for (key, val) in hashmap.iter() {
                        tile_vec.push((
                            MapCoordinate::from(*key),
                            val.as_ref().unwrap().content.get_value().0.unwrap(),
                        ));
                    }
                    // find the tile coordinate corresponding to the max value
                    let result = tile_vec.iter().max_by_key(|x| x.1).cloned().unwrap();
                    // return the result
                    Ok(Some(result))
                }
                Err(error) => {
                    return match error {
                        LibError::NotEnoughEnergy => Err(Box::new(ToolError::NotEnoughEnergy)),
                        LibError::NoMoreDiscovery => Err(Box::new(ToolError::NoMoreDiscovery)),
                        other => Err(Box::new(ToolError::Other(format!("{:?}", other)))),
                    }
                }
            };
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
        /// ```ignore
        ///
        /// // Create objects and define pattern
        /// use resource_scanner_tool::tool::resource_scanner::*;
        /// let mut robot = create_robot();
        /// let world = create_world();
        /// let pattern = Pattern::Area(3);
        ///
        /// // Get target coordinates
        /// let coordinates = get_coordinates(&mut robot, &world, &pattern);
        /// println!("{:?}", coordinates);
        /// ```
        fn get_target_coordinates(
            robot: &mut impl Runnable,
            world: &World,
            pattern: &Pattern,
        ) -> Option<Vec<MapCoordinate>> {
            let mut out = Vec::new();
            let world_size = robot_map(world).unwrap().len();
            let (y_robot, x_robot) = (
                robot.get_coordinate().get_row(),
                robot.get_coordinate().get_col(),
            );

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
                            if !(x_world < 0
                                || x_world > (world_size as i32) - 1
                                || y_world < 0
                                || y_world > (world_size as i32) - 1)
                            {
                                out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                            }
                        }
                    }
                }

                Pattern::DirectionLeft(size) => {
                    let length = *size as i32;
                    let y_world = y_robot as i32;
                    for x in 0..=-length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let x_world = (x_robot as i32) + x;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DirectionRight(size) => {
                    let length = *size as i32;
                    let y_world = y_robot as i32;
                    for x in 0..=length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let x_world = (x_robot as i32) + x;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DirectionUp(size) => {
                    let length = *size as i32;
                    let x_world = x_robot as i32;
                    for y in 0..=length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DirectionDown(size) => {
                    let length = *size as i32;
                    let x_world = x_robot as i32;
                    for y in 0..=-length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalUpperLeft(size) => {
                    let length = *size as i32;
                    for i in 0..=length {
                        let x = -i;
                        let y = -i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalUpperRight(size) => {
                    let length = *size as i32;
                    for i in 0..=length {
                        let x = i;
                        let y = -i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalLowerLeft(size) => {
                    let length = *size as i32;
                    for i in 0..=length {
                        let x = -i;
                        let y = i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalLowerRight(size) => {
                    let length = *size as i32;
                    for i in 0..=length {
                        let x = i;
                        let y = i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalStar(size) => {
                    let length = *size as i32;
                    //push robot coordinates
                    out.push(MapCoordinate::new(x_robot, y_robot));
                    //push rest of coordinates
                    for i in 1..=length {
                        for multiplier in [(1, 1), (1, -1), (-1, 1), (1, 1)] {
                            let x = multiplier.0 * i;
                            let y = multiplier.1 * i;
                            // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                            let x_world = (x_robot as i32) + x;
                            let y_world = (y_robot as i32) + y;
                            // check if the coordinates are out of bound, if so omit them
                            if !(x_world < 0
                                || x_world > (world_size as i32) - 1
                                || y_world < 0
                                || y_world > (world_size as i32) - 1)
                            {
                                out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                            }
                        }
                    }
                }

                Pattern::StraightStar(size) => {
                    let length = *size as i32;

                    // horizontal arms
                    let y_world = y_robot as i32;
                    for x in -length..=length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let x_world = (x_robot as i32) + x;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }

                    // vertical upper arm
                    let x_world = x_robot as i32;
                    for y in 1..=length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }

                    // vertical lower arm
                    for y in -length..0 {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0
                            || x_world > (world_size as i32) - 1
                            || y_world < 0
                            || y_world > (world_size as i32) - 1)
                        {
                            out.push(MapCoordinate::new(x_world as usize, y_world as usize));
                        }
                    }
                }
            }

            return if out.len() == 0 { None } else { Some(out) };
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
        /// ```ignore
        /// use resource_scanner_tool::tool::*;
        /// let mut robot = create_robot();
        /// let world = create_world();
        /// let pattern = Pattern::Area(3);
        ///
        /// // Get sanitized coordinates
        /// let sanitized_coordinates = get_sanitized_tiles(&mut robot, &world, &pattern);
        /// println!("{:?}", sanitized_coordinates);
        /// ```
        fn get_sanitized_tiles(
            robot: &mut impl Runnable,
            world: &World,
            pattern: &Pattern,
        ) -> Vec<MapCoordinate> {
            let target_vector = ResourceScanner::get_target_coordinates(robot, world, pattern);

            return match target_vector {
                Some(mut v) => {
                    let mut tiles_to_remove = Vec::new();
                    let known_coordinates = robot_map(world).unwrap();
                    for (index, coordinate) in v.iter().enumerate() {
                        if known_coordinates[coordinate.get_width()][coordinate.get_height()]
                            .is_some()
                        {
                            tiles_to_remove.push(index);
                        }
                    }
                    // sort and then iterate in inverse order
                    tiles_to_remove.sort();
                    for index in tiles_to_remove.iter().rev() {
                        v.remove(*index);
                    }
                    v
                }
                None => Vec::new(),
            };
        }
    }
}
