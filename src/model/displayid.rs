#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayIdProductDataBlock {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayIdVendorSpecificDataBlock {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayIdDataBlockKind {
    Product(DisplayIdProductDataBlock),
    VendorSpecific(DisplayIdVendorSpecificDataBlock),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayIdDataBlock {
    pub tag: u8,
    pub revision: u8,
    pub payload: Vec<u8>,
    pub kind: DisplayIdDataBlockKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayIdExtension {
    pub version: u8,
    pub revision: u8,
    pub payload_bytes: u8,
    pub product_type: u8,
    pub extension_count: u8,
    pub data_blocks: Vec<DisplayIdDataBlock>,
    pub raw_block: Vec<u8>,
}
