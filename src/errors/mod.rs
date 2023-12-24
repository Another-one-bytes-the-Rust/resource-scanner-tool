pub mod tool_errors {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub enum ToolError {
        InvalidSizeError,
        EmptyCoordinates,
        NotEnoughEnergy,
        NoMoreDiscovery,
        ContentNotSupported,
        Other(String),
    }

    impl Debug for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl Display for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return match self {
                ToolError::EmptyCoordinates => write!(f,"{}","Empty Coordinates".to_string()),
                ToolError::NotEnoughEnergy => write!(f,"{}","Not Enough Energy".to_string()),
                ToolError::Other(message) => write!(f, "{}", message),
                ToolError::InvalidSizeError => write!(f,"Invalid Size"),
                ToolError::NoMoreDiscovery => write!(f,"{}","No More Discovery".to_string()),
                ToolError::ContentNotSupported => write!(f,"{}","The used content is not supported".to_string())
            };
        }
    }

    impl Error for ToolError {}
}
