mod parse;
mod types;
mod write;

pub use types::{
    AudioBlock, Cta861Extension, DataBlock, ExtendedTagBlock, HdmiVendorBlock,
    HdrStaticMetadataBlock, SpeakerAllocationBlock, VendorBlock, VideoBlock,
};

pub(crate) use parse::parse_cta861_extension;
pub(crate) use write::write_cta861_extension;
