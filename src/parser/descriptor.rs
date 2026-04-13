use crate::{
    error::EdidError,
    model::{Descriptor, DetailedTiming, RangeLimits},
    utils::BitReader,
};

pub(crate) fn parse_descriptor(bytes: &[u8]) -> Result<Descriptor, EdidError> {
    if bytes.len() != 18 {
        return Err(EdidError::ParseError(
            "descriptor must be 18 bytes".to_owned(),
        ));
    }

    let mut raw = [0_u8; 18];
    raw.copy_from_slice(bytes);

    if bytes[0] != 0 || bytes[1] != 0 {
        let pixel_clock_khz = u32::from(u16::from_le_bytes([bytes[0], bytes[1]])) * 10;
        let horizontal_active =
            u16::from(bytes[2]) | (u16::from(BitReader::high_nibble(bytes[4])) << 8);
        let horizontal_blanking =
            u16::from(bytes[3]) | (u16::from(BitReader::low_nibble(bytes[4])) << 8);
        let vertical_active =
            u16::from(bytes[5]) | (u16::from(BitReader::high_nibble(bytes[7])) << 8);
        let vertical_blanking =
            u16::from(bytes[6]) | (u16::from(BitReader::low_nibble(bytes[7])) << 8);

        return Ok(Descriptor::DetailedTiming(DetailedTiming {
            pixel_clock_khz,
            horizontal_active,
            horizontal_blanking,
            vertical_active,
            vertical_blanking,
            raw,
        }));
    }

    match bytes[3] {
        0xfc => Ok(Descriptor::MonitorName(
            crate::utils::parse_descriptor_text(&bytes[5..18]),
        )),
        0xff => Ok(Descriptor::MonitorSerial(
            crate::utils::parse_descriptor_text(&bytes[5..18]),
        )),
        0xfd => Ok(Descriptor::RangeLimits(RangeLimits {
            min_vertical_hz: bytes[5],
            max_vertical_hz: bytes[6],
            min_horizontal_khz: bytes[7],
            max_horizontal_khz: bytes[8],
            max_pixel_clock_mhz: u16::from(bytes[9]) * 10,
            raw,
        })),
        _ => Ok(Descriptor::Unknown(raw)),
    }
}
