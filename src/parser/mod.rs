mod base;
mod cta861;
mod descriptor;
mod displayid;

use crate::{
    error::EdidError,
    model::{Edid, ExtensionBlock},
};

pub(crate) use base::parse_base_block;

pub(crate) fn parse_edid(data: &[u8]) -> Result<Edid, EdidError> {
    if data.len() < 128 || data.len() % 128 != 0 {
        return Err(EdidError::InvalidLength);
    }

    crate::utils::validate_header(data)?;

    let actual_extensions = (data.len() / 128).saturating_sub(1);
    let base = parse_base_block(&data[..128])?;

    if usize::from(base.extension_count) != actual_extensions {
        return Err(EdidError::ExtensionCountMismatch {
            expected: usize::from(base.extension_count),
            actual: actual_extensions,
        });
    }

    let mut extensions = Vec::with_capacity(actual_extensions);
    for (index, block) in data[128..].chunks_exact(128).enumerate() {
        crate::utils::validate_checksum(block, index + 1)?;

        let extension = match block[0] {
            0x02 => ExtensionBlock::Cta861(cta861::parse_cta861_extension(block)?),
            0x70 => ExtensionBlock::DisplayId(displayid::parse_displayid_extension(block)?),
            _ => ExtensionBlock::Unknown(block.to_vec()),
        };
        extensions.push(extension);
    }

    Ok(Edid {
        raw: data.to_vec(),
        base,
        extensions,
    })
}
