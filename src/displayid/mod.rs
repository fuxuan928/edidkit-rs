mod parse;
mod types;
mod write;

pub use types::{
    DisplayIdDataBlock, DisplayIdDataBlockKind, DisplayIdExtension, DisplayIdProductDataBlock,
    DisplayIdVendorSpecificDataBlock,
};

pub(crate) use parse::parse_displayid_extension;
pub(crate) use write::write_displayid_extension;
