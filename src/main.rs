pub mod resource_scanner {
    use std::collections::HashMap;
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::iter::Map;
    use std::num::Wrapping;
    use std::ops::{Add, Sub};
    use robotics_lib::interface::{Tools, robot_map, robot_view, discover_tiles, one_direction_view};
    use robotics_lib::runner::Runnable;
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use robotics_lib::utils::LibError;
    use robotics_lib::utils::LibError::NoMoreDiscovery;
    use crate::resource_scanner::ToolError::{EmptyCoordinates, InvalidSizeError, NotEnoughEnergy};

    pub struct ResourceScanner {}
    impl Tools for ResourceScanner {}

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

    /// The `MapCoordinate` struct represents coordinates within a two-dimensional map or grid.
    ///
    /// ## Fields
    ///
    /// - `width`: An unsigned integer representing the number of columns in the coordinate system.
    /// - `height`: An unsigned integer representing the number of rows in the coordinate system.
    ///
    /// ## Example
    ///
    /// ```
    /// // Creating a new MapCoordinate instance
    /// let coordinate = MapCoordinate::new(8,3);
    ///
    /// // Accessing width and height
    /// println!("Width: {}", coordinate.get_width());
    /// println!("Height: {}", coordinate.get_height());
    /// ```
    ///
    pub struct MapCoordinate {
        width: usize,
        height: usize
    }
    impl MapCoordinate {
        /// Creates a new `MapCoordinate` instance with the given width and height.
        ///
        /// # Arguments
        ///
        /// * `width` - The width of the map.
        /// * `height` - The height of the map.
        ///
        /// # Example
        ///
        /// ```
        /// let coordinates = MapCoordinate::new(10, 20);
        /// ```
        pub fn new(width: usize, height: usize) -> Self {
            MapCoordinate { width, height }
        }

        /// Gets the width of the map coordinate.
        ///
        /// # Example
        ///
        /// ```
        /// let coordinates = MapCoordinate::new(10, 20);
        /// let width = coordinates.get_width();
        /// assert_eq!(width, 10);
        /// ```
        pub fn get_width(&self) -> usize {
            self.width
        }

        /// Sets the width of the map coordinate.
        ///
        /// # Arguments
        ///
        /// * `width` - The new width value.
        ///
        /// # Example
        ///
        /// ```
        /// let mut coordinates = MapCoordinate::new(10, 20);
        /// coordinates.set_width(15);
        /// assert_eq!(coordinates.get_width(), 15);
        /// ```
        pub fn set_width(&mut self, width: usize) {
            self.width = width;
        }

        /// Gets the height of the map coordinate.
        ///
        /// # Example
        ///
        /// ```
        /// let coordinates = MapCoordinate::new(10, 20);
        /// let height = coordinates.get_height();
        /// assert_eq!(height, 20);
        /// ```
        pub fn get_height(&self) -> usize {
            self.height
        }

        /// Sets the height of the map coordinate.
        ///
        /// # Arguments
        ///
        /// * `height` - The new height value.
        ///
        /// # Example
        ///
        /// ```
        /// let mut coordinates = MapCoordinate::new(10, 20);
        /// coordinates.set_height(25);
        /// assert_eq!(coordinates.get_height(), 25);
        /// ```
        pub fn set_height(&mut self, height: usize) {
            self.height = height;
        }
    }
    
    impl PartialEq for MapCoordinate {
        fn eq(&self, other: &Self) -> bool {
            self.height == other.height && self.width == other.width
        }
    }

    impl Add for MapCoordinate {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                width: self.width + rhs.width,
                height: self.height + rhs.height
            }
        }
    }

    impl Sub for MapCoordinate {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                width: self.width - rhs.width,
                height: self.height - rhs.height
            }
        }
    }

    impl From<(usize,usize)> for MapCoordinate {
        fn from(value: (usize, usize)) -> Self {
            Self {
                width: value.0,
                height: value.1
            }
        }
    }

    impl Into<(usize,usize)> for MapCoordinate {
        fn into(self) -> (usize, usize) {
            (self.width, self.height)
        }
    }

    pub enum ToolError{
        InvalidSizeError,
        EmptyCoordinates,
        NotEnoughEnergy,
        NoMoreDiscovery,
        UnknownError,

    }

    impl Debug for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl Display for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl Error for ToolError {
    }


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
        ) -> Result<Option<(MapCoordinate, usize)>,Box<dyn Error>> {

            // first check if any of the tiles in the scan pattern are already present in the robot map
            let coordinates_to_check: () = match pattern {
                //todo IMPORTANT, CHECK IN EACH CASE 'size' FOR INVALID VALUES
                Pattern::Area(size) => {
                    // return an error if the given size doesn't allow for a square area
                    if size % 2 == 0 || size < 3{
                        return Err(Box::new(InvalidSizeError));
                    }
                    let target_coordinates = ResourceScanner::get_target_coordinates(robot,world,&Pattern::Area(size))
                        .ok_or(Box::new(EmptyCoordinates))?;

                    let known_coordinates = robot_map(world).unwrap();

                    let sanitized_coordinates = ResourceScanner::get_sanitized_tiles(robot,world,&pattern);

                    // todo() this does not have a size known at compile time, thus it cannot be used in discover_tiles, a possible solution is
                    // todo() limiting the max size of the coordinate vector
                    let sanitized_coordinates_as_slice = sanitized_coordinates.iter().map(|x|x as (usize,usize)).collect();


                    let tiles = discover_tiles(robot, world, &sanitized_coordinates_as_slice);

                    return match tiles {
                        Ok(mut hashmap) => {
                            // retain only the tiles containing the requested content
                            hashmap.retain(|key,val| val.unwrap().content == content);

                            // create a v/home/kirk/Courses/Advanced-Programming/Project/tile-resource-mapper-toolector containing tile coordinates and corresponding content quantity
                            let mut tile_vec: Vec<(MapCoordinate,usize)> = Vec::new();
                            for (key,val) in hashmap.iter() {
                                tile_vec.push((key as MapCoordinate, val.unwrap().content.get_value().0.unwrap()));
                            }
                            // find the tile coordinate corresponding to the max value
                            let result = tile_vec.iter().max_by_key(|x|x.1);
                            Ok(*result)
                        },
                        Err(error) => {
                            return match error {
                                LibError::NotEnoughEnergy => Err(Box::new(ToolError::NotEnoughEnergy)),
                                LibError::NoMoreDiscovery => Err(Box::new(ToolError::NotEnoughEnergy)),
                                _ =>  Err(Box::new(ToolError::UnknownError)),
                            }
                        }
                    }

                }
                _ => Vec::new()
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
        /// Returns an `Option<Vec<MapCoordinate>>` representing the vector of target coordinates.
        /// Returns `None` if no valid coordinates are found.
        ///
        /// # Examples
        ///
        /// ```
        /// use resource-scanner-tool::{Runnable, World, Pattern, MapCoordinate, get_coordinates};
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
                    let x_area_robot = length/2;
                    let y_area_robot = length/2;
                    for x in 0..length {
                        for y in 0..length {
                            // compute the tile coordinates in the world FoR (Frame of Reference) from the tile coordinates in the area FoR
                            let x_world = (x_robot as i32) + x - x_area_robot;
                            let y_world = (y_robot as i32) + y - y_area_robot;
                            // check if the coordinates are out of bound, if so omit them
                            if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                                out.push(MapCoordinate::new(x_world as usize,y_world as usize));
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
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
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
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
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
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
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
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalUpperLeft(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = -i;
                        let y= -i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalUpperRight(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = i;
                        let y= -i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalLowerLeft(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = -i;
                        let y= i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalLowerRight(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        let x = i;
                        let y= i;
                        // compute the tile coordinates in the world FoR from the tile coordinates in the area FoR
                        let x_world = (x_robot as i32) + x;
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
                        }
                    }
                }

                Pattern::DiagonalStar(size) => {
                    let length = *size as i32;
                    for i in 0..length {
                        for multiplier in [(1,1),(1,-1),(-1,1),(1,1)] {
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
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
                        }
                    }

                    // vertical arms
                    let x_world = x_robot as i32;
                    for y in -length..length {
                        // compute the tile coordinates in the world FoR from the tile coordinates in the robot FoR
                        let y_world = (y_robot as i32) + y;
                        // check if the coordinates are out of bound, if so omit them
                        if !(x_world < 0 || x_world > (world_size as i32)-1 || y_world < 0 || y_world > (world_size as i32)-1) {
                            out.push(MapCoordinate::new(x_world as usize,y_world as usize));
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
        /// Returns a vector of `MapCoordinate` representing the sanitized coordinates.
        ///
        /// # Errors
        ///
        /// Returns an empty vector if no target coordinates are found.
        ///
        /// # Examples
        ///
        /// ```
        /// use resource-scanner-tool::{Runnable, World, Pattern, MapCoordinate, get_sanitized_tiles};
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
                if known_coordinates[coordinate.height][coordinate.width].is_none() {
                    target_vector.remove(index);
                }
            }
            target_vector
        }
    }
}

fn main() {

}