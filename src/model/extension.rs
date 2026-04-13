use super::{cta861::Cta861Extension, displayid::DisplayIdExtension};

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionBlock {
    Cta861(Cta861Extension),
    DisplayId(DisplayIdExtension),
    Unknown(Vec<u8>),
}

impl ExtensionBlock {
    pub fn raw_bytes(&self) -> &[u8] {
        match self {
            Self::Cta861(cta) => &cta.raw_block,
            Self::DisplayId(display_id) => &display_id.raw_block,
            Self::Unknown(bytes) => bytes,
        }
    }
}
