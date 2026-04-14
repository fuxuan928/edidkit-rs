mod descriptor;
mod parse;
mod types;
mod write;

pub use descriptor::{Descriptor, DetailedTiming, RangeLimits};
pub use types::{
    AnalogVideoInput, BaseBlock, DigitalVideoInput, EdidVersion, ManufactureDate, ManufacturerId,
    VideoInputDefinition,
};

pub(crate) use parse::parse_base_block;
pub(crate) use write::write_base_block;
