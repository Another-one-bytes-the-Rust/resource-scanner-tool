
pub mod resource_scanner {
    use robotics_lib::interface::{Tools, robot_map, robot_view, discover_tiles, one_direction_view};
    use robotics_lib::runner::Runnable;
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use robotics_lib::utils::LibError;

    pub struct ResourceScanner {}

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



    impl Tools for ResourceScanner {}

    impl ResourceScanner {
        /// The scan function scans an area around the robot for the required content according to the pattern.
        /// it returns an option containing tile coordinates and number of contents if some content is found.
        /// if no content is found in the given area it returns the origin as coordinate and 0 as quantity.
        /// if the robot didn't have enough energy it returns None
        pub fn scan(&mut self,
                    world: &World,
                    robot: &mut impl Runnable,
                    pattern: Pattern,
                    content: Content
        ) -> Option<(Coordinate, usize)> {
            // first check if any of the tiles in the scan pattern are already present in the robot map
            let coordinates_to_check: Vec<Coordinate> = match pattern {
                Pattern::Area(size) => {
                    //todo check if size is odd, if not return an error
                    let tile_map = robot_view(robot,world);
                    let out = Vec::new();

                    for (height_idx,row_vec) in tile_map.iter().enumerate() {
                        for (width_idx, tile) in row_vec.iter().enumerate() {
                            out.push(Coordinate::new())
                        }
                    }

                }
                _ => Vec::new()
            }
        }


    }
}

fn main() {}