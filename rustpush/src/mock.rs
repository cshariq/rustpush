#[cfg(not(target_os = "macos"))]
pub mod open_absinthe {
    use std::fmt;
    use std::error::Error;

    pub mod nac {
        #[derive(Debug, Clone, PartialEq)]
        pub struct HardwareConfig {}
        
        impl HardwareConfig {
            pub fn new() -> Self {
                HardwareConfig {}
            }
        }
        
        #[derive(Debug, Clone, PartialEq)]
        pub struct ValidationCtx {}
        
        impl ValidationCtx {
            pub fn new(_config: &HardwareConfig) -> Self {
                ValidationCtx {}
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsintheError {
        pub message: String,
    }

    impl AbsintheError {
        pub fn new(message: String) -> Self {
            Self { message }
        }
    }
    
    impl fmt::Display for AbsintheError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Mock AbsintheError: {}", self.message)
        }
    }
    
    impl Error for AbsintheError {}
}
