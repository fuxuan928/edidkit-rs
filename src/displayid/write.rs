use crate::{displayid::DisplayIdExtension, error::EdidError};

pub(crate) fn write_displayid_extension(
    display_id: &DisplayIdExtension,
) -> Result<Vec<u8>, EdidError> {
    Ok(display_id.raw_block.clone())
}
