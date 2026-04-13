use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdidError {
    InvalidLength,
    InvalidHeader,
    InvalidChecksum { block_index: usize },
    ExtensionCountMismatch { expected: usize, actual: usize },
    UnsupportedExtension(u8),
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for EdidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid EDID length"),
            Self::InvalidHeader => write!(f, "invalid EDID header"),
            Self::InvalidChecksum { block_index } => {
                write!(f, "invalid checksum for block {block_index}")
            }
            Self::ExtensionCountMismatch { expected, actual } => {
                write!(
                    f,
                    "extension count mismatch: expected {expected}, got {actual}"
                )
            }
            Self::UnsupportedExtension(tag) => write!(f, "unsupported extension tag 0x{tag:02x}"),
            Self::ParseError(message) => write!(f, "parse error: {message}"),
            Self::ValidationError(message) => write!(f, "validation error: {message}"),
        }
    }
}

impl std::error::Error for EdidError {}
