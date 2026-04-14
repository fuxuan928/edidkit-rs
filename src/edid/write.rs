use crate::{
    base::write_base_block, cta861::write_cta861_extension, displayid::write_displayid_extension,
    error::EdidError,
};

use super::{Edid, ExtensionBlock};

pub(crate) fn write_edid(edid: &Edid) -> Result<Vec<u8>, EdidError> {
    let mut out = Vec::with_capacity(128 * (1 + edid.extensions.len()));
    out.extend_from_slice(&write_base_block(&edid.base)?);

    for extension in &edid.extensions {
        match extension {
            ExtensionBlock::Cta861(cta) => out.extend_from_slice(&write_cta861_extension(cta)?),
            ExtensionBlock::DisplayId(display_id) => {
                out.extend_from_slice(&write_displayid_extension(display_id)?)
            }
            ExtensionBlock::Unknown(bytes) => out.extend_from_slice(bytes),
        }
    }

    Ok(out)
}
