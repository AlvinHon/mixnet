use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixnetError {
    InvalidParameter(&'static str),
    RingMismatch,
    EmptyInput,
}

impl fmt::Display for MixnetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter(msg) => write!(f, "invalid parameter: {msg}"),
            Self::RingMismatch => write!(f, "ring mismatch"),
            Self::EmptyInput => write!(f, "empty input"),
        }
    }
}

impl std::error::Error for MixnetError {}

pub type MixnetResult<T> = Result<T, MixnetError>;
