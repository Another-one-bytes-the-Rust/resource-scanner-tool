pub mod tool_errors {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub enum ToolError{
        InvalidSizeError,
        EmptyCoordinates,
        NotEnoughEnergy,
        NoMoreDiscovery,
        Other(String),

    }

    impl Debug for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl Display for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return match self {
                ToolError::EmptyCoordinates => write!(f,""),
                ToolError::NotEnoughEnergy => write!(f,""),
                ToolError::Other(message) => write!(f,"{}", message),
                ToolError::InvalidSizeError => write!(f,""),
                ToolError::NoMoreDiscovery => write!(f,""),

            }
        }
    }

    impl Error for ToolError {
    }
}

pub mod map_coordinate {
    use std::ops::{Add, Sub};


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
    /// use resource_scanner_tool::map_coordinate::MapCoordinate;
    /// let coordinate = MapCoordinate::new(8,3);
    ///
    /// // Accessing width and height
    /// println!("Width: {}", coordinate.get_width());
    /// println!("Height: {}", coordinate.get_height());
    /// ```
    ///
    #[derive(Debug,Clone,Copy)]
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
        /// use resource_scanner_tool::map_coordinate::MapCoordinate;
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
        /// use resource_scanner_tool::map_coordinate::MapCoordinate;
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
        /// use resource_scanner_tool::map_coordinate::MapCoordinate;
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
        /// use resource_scanner_tool::map_coordinate::MapCoordinate;
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
        /// use resource_scanner_tool::map_coordinate::MapCoordinate;
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
}