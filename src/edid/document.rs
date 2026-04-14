use crate::{
    base::{BaseBlock, Descriptor},
    cta861::Cta861Extension,
    displayid::DisplayIdExtension,
    error::EdidError,
};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Edid {
    pub raw: Vec<u8>,
    pub base: BaseBlock,
    pub extensions: Vec<ExtensionBlock>,
}

impl Edid {
    pub fn parse(data: &[u8]) -> Result<Self, EdidError> {
        super::parse::parse_edid(data)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        super::write::write_edid(self)
    }

    pub fn set_product_code(&mut self, code: u16) {
        self.base.product_code = code;
    }

    pub fn set_monitor_name(&mut self, name: &str) -> Result<(), EdidError> {
        crate::utils::validate_descriptor_text(name)?;

        for descriptor in &mut self.base.descriptors {
            if let Descriptor::MonitorName(existing) = descriptor {
                *existing = name.to_owned();
                return Ok(());
            }
        }

        Err(EdidError::ValidationError(
            "monitor name descriptor not present".to_owned(),
        ))
    }
}
