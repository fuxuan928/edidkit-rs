use super::descriptor::Descriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManufacturerId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManufactureDate {
    pub week: u8,
    pub year: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalogVideoInput {
    pub separate_sync_supported: bool,
    pub composite_sync_on_hsync_supported: bool,
    pub composite_sync_on_green_supported: bool,
    pub serration_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalVideoInput {
    pub dfp_1x_compatible: bool,
    pub color_bit_depth: Option<u8>,
    pub interface: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoInputDefinition {
    Analog(AnalogVideoInput),
    Digital(DigitalVideoInput),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseBlock {
    pub manufacturer_id: ManufacturerId,
    pub product_code: u16,
    pub serial_number: u32,
    pub manufacture_date: ManufactureDate,
    pub version: super::edid::EdidVersion,
    pub video_input_definition: VideoInputDefinition,
    pub extension_count: u8,
    pub descriptors: Vec<Descriptor>,
    pub raw_block: [u8; 128],
}
