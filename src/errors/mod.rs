pub mod tool_errors {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub enum ToolError {
        InvalidSizeError,
        EmptyCoordinates,
        NotEnoughEnergy,
        NoMoreDiscovery,
        Other(String),
    }

    impl Debug for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Display for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return match self {
                ToolError::EmptyCoordinates => write!(f, ""),
                ToolError::NotEnoughEnergy => write!(f, ""),
                ToolError::Other(message) => write!(f, "{}", message),
                ToolError::InvalidSizeError => write!(f, ""),
                ToolError::NoMoreDiscovery => write!(f, ""),
            };
        }
    }

    impl Error for ToolError {}
}
