pub mod cli;
pub mod format;
pub mod structure;
pub mod system;

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum VroomError {
        #[error("There is no item in '{1}' named '{0}'")]
        NoSuchItem(String, String),

        #[error("There is no item named '{0}'")]
        NoSuchItemAny(String),

        #[error("There is no list named '{0}'")]
        NoSuchList(String),

        #[error("You didn't provide a required argument: {0}")]
        MissingArgument(String),

        #[error("There was an error in saving the vroomfile")]
        SaveError(#[from] std::io::Error),

        #[error("There was an error in loading the vroomfile")]
        LoadError(#[from] serde_json::Error),
    }

    pub type Result = std::result::Result<(), VroomError>;
}
