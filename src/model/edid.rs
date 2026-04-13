use crate::{error::EdidError, parser, writer};

use super::{BaseBlock, ExtensionBlock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdidVersion {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edid {
    pub raw: Vec<u8>,
    pub base: BaseBlock,
    pub extensions: Vec<ExtensionBlock>,
}

impl Edid {
    pub fn parse(data: &[u8]) -> Result<Self, EdidError> {
        parser::parse_edid(data)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        writer::write_edid(self)
    }

    pub fn set_product_code(&mut self, code: u16) {
        self.base.product_code = code;
    }

    pub fn set_monitor_name(&mut self, name: &str) -> Result<(), EdidError> {
        crate::utils::validate_descriptor_text(name)?;

        for descriptor in &mut self.base.descriptors {
            if let super::Descriptor::MonitorName(existing) = descriptor {
                *existing = name.to_owned();
                return Ok(());
            }
        }

        Err(EdidError::ValidationError(
            "monitor name descriptor not present".to_owned(),
        ))
    }
}
