pub mod resource_scanner {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::num::Wrapping;
    use robotics_lib::interface::{Tools, robot_map, robot_view, discover_tiles, one_direction_view};
    use robotics_lib::runner::Runnable;
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use robotics_lib::utils::LibError;
    use crate::resource_scanner::ToolError::InvalidSizeError;

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

    pub enum ToolError{
        InvalidSizeError,

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
                    world: &World,
                    robot: &mut impl Runnable,
                    pattern: Pattern,
                    content: Content
        ) -> Result<Option<(MapCoordinate, usize)>,Box<dyn Error>> {
            // first check if any of the tiles in the scan pattern are already present in the robot map
            let coordinates_to_check: Vec<MapCoordinate> = match pattern {
                Pattern::Area(size) => {
                    // return an error if the given size doesn't allow for a square area
                    if size % 2 == 0 || size < 3{
                        return Err(Box::new(InvalidSizeError));
                    }

                    let

                    let mut out = Vec::new();

                    for (height_idx,row_vec) in robot_map.iter().enumerate() {
                        for (width_idx, tile) in row_vec.iter().enumerate() {
                            out.push(MapCoordinate::new(width_idx,height_idx))
                        }
                    }
                    out
                }
                _ => Vec::new()
            };
        }

        fn get_coordinates(robot: &mut impl Runnable, world: &World, pattern: &Pattern) -> Option<Vec<MapCoordinate>> {
            let mut out = Vec::new();
            let world_size = robot_map(world).unwrap().len();
            let (x_robot, y_robot) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
            match pattern {
                Pattern::Area(size) => {
                    let lower_bound = size/2;
                    let upper_bound = size/2 + 1;
                    for x in lower_bound..upper_bound {
                        for y in lower_bound..upper_bound {
                            // check if the coordinates are out of bound, if so omit them
                            if !(x < 0 || x > world_size-1 || y < 0 || y > world_size-1) {
                                // compute the tile world coordinates given the robot coordinates
                                let x_world = x_robot - x;
                                let y_world = y_robot - y;
                            }
                        }
                    }

                }
            }
        }
    }
}

fn main() {

}