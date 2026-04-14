use crate::displayid::DisplayIdExtension;

pub(crate) fn write_displayid_extension(display_id: &DisplayIdExtension) -> Vec<u8> {
    display_id.raw_block.clone()
}
