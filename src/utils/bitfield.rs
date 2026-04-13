pub(crate) struct BitReader;

impl BitReader {
    pub(crate) fn high_nibble(byte: u8) -> u8 {
        byte >> 4
    }

    pub(crate) fn low_nibble(byte: u8) -> u8 {
        byte & 0x0f
    }
}
