#[cfg(test)]
mod tests {
    use crate::coordinates::map_coordinate::MapCoordinate;
    use crate::errors::tool_errors::ToolError;

    #[test]
    fn test_new_map_coordinate() {
        let coordinates = MapCoordinate::new(10, 20);
        assert_eq!(coordinates.get_width(), 10);
        assert_eq!(coordinates.get_height(), 20);
    }

    #[test]
    fn test_get_width() {
        let coordinates = MapCoordinate::new(10, 20);
        assert_eq!(coordinates.get_width(), 10);
    }

    #[test]
    fn test_set_width() {
        let mut coordinates = MapCoordinate::new(10, 20);
        coordinates.set_width(15);
        assert_eq!(coordinates.get_width(), 15);
    }

    #[test]
    fn test_get_height() {
        let coordinates = MapCoordinate::new(10, 20);
        assert_eq!(coordinates.get_height(), 20);
    }

    #[test]
    fn test_set_height() {
        let mut coordinates = MapCoordinate::new(10, 20);
        coordinates.set_height(25);
        assert_eq!(coordinates.get_height(), 25);
    }

    #[test]
    fn test_equality() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(10, 20);
        let coordinates3 = MapCoordinate::new(15, 25);

        assert_eq!(coordinates1, coordinates2);
        assert_ne!(coordinates1, coordinates3);
    }

    #[test]
    fn test_addition() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(5, 10);
        let result = coordinates1 + coordinates2;
        assert_eq!(result, MapCoordinate::new(15, 30));
    }

    #[test]
    fn test_subtraction() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(5, 10);
        let result = coordinates1 - coordinates2;
        assert_eq!(result, MapCoordinate::new(5, 10));
    }

    #[test]
    fn test_from_into_conversion() {
        let tuple_coordinates: (usize, usize) = (10, 20);
        let coordinates: MapCoordinate = tuple_coordinates.into();
        assert_eq!(coordinates.get_width(), 10);
        assert_eq!(coordinates.get_height(), 20);

        let converted_tuple: (usize, usize) = coordinates.into();
        assert_eq!(converted_tuple, (10, 20));

    }

    #[test]
    fn test_debug_display_and_error_traits() {
        // Test Debug trait
        assert_eq!(
            format!("{:?}", ToolError::InvalidSizeError),
            "InvalidSizeError"
        );
        assert_eq!(
            format!("{:?}", ToolError::EmptyCoordinates),
            "EmptyCoordinates"
        );
        assert_eq!(
            format!("{:?}", ToolError::NotEnoughEnergy),
            "NotEnoughEnergy"
        );
        assert_eq!(
            format!("{:?}", ToolError::NoMoreDiscovery),
            "NoMoreDiscovery"
        );
        assert_eq!(
            format!("{:?}", ToolError::Other("Custom Error".to_string())),
            "Other(\"Custom Error\")"
        );

        // Test Display trait
        assert_eq!(
            format!("{}", ToolError::InvalidSizeError),
            "InvalidSizeError"
        );
        assert_eq!(
            format!("{}", ToolError::EmptyCoordinates),
            "EmptyCoordinates"
        );
        assert_eq!(
            format!("{}", ToolError::NotEnoughEnergy),
            "NotEnoughEnergy"
        );
        assert_eq!(
            format!("{}", ToolError::NoMoreDiscovery),
            "NoMoreDiscovery"
        );
        assert_eq!(
            format!("{}", ToolError::Other("Custom Error".to_string())),
            "Custom Error"
        );

        // Test Error trait
        assert_eq!(
            ToolError::InvalidSizeError.to_string(),
            "InvalidSizeError"
        );
        assert_eq!(
            ToolError::EmptyCoordinates.to_string(),
            "EmptyCoordinates"
        );
        assert_eq!(
            ToolError::NotEnoughEnergy.to_string(),
            "NotEnoughEnergy"
        );
        assert_eq!(
            ToolError::NoMoreDiscovery.to_string(),
            "NoMoreDiscovery"
        );
        assert_eq!(
            ToolError::Other("Custom Error".to_string()).to_string(),
            "Custom Error"
        );
    }
}
