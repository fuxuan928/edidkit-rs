mod error;
mod model;
mod parser;
mod utils;
mod writer;

pub use error::EdidError;
pub use model::{
    AnalogVideoInput, AudioBlock, BaseBlock, Cta861Extension, DataBlock, Descriptor,
    DetailedTiming, DigitalVideoInput, DisplayIdDataBlock, DisplayIdDataBlockKind,
    DisplayIdExtension, DisplayIdProductDataBlock, DisplayIdVendorSpecificDataBlock, Edid,
    EdidVersion, ExtendedTagBlock, ExtensionBlock, HdmiVendorBlock, HdrStaticMetadataBlock,
    ManufactureDate, ManufacturerId, RangeLimits, SpeakerAllocationBlock, VendorBlock, VideoBlock,
    VideoInputDefinition,
};
