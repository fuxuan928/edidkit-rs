mod base;
mod cta861;

use crate::model::{Edid, ExtensionBlock};

pub(crate) fn write_edid(edid: &Edid) -> Vec<u8> {
    let mut out = Vec::with_capacity(128 * (1 + edid.extensions.len()));
    out.extend_from_slice(&base::write_base_block(&edid.base));

    for extension in &edid.extensions {
        match extension {
            ExtensionBlock::Cta861(cta) => {
                out.extend_from_slice(&cta861::write_cta861_extension(cta))
            }
            ExtensionBlock::DisplayId(display_id) => out.extend_from_slice(&display_id.raw_block),
            ExtensionBlock::Unknown(bytes) => out.extend_from_slice(bytes),
        }
    }

    out
}
