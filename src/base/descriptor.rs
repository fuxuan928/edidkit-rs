use crate::{error::EdidError, utils::BitReader};

#[derive(Debug, Clone, PartialEq)]
pub struct DetailedTiming {
    pub pixel_clock_khz: u32,
    pub horizontal_active: u16,
    pub horizontal_blanking: u16,
    pub vertical_active: u16,
    pub vertical_blanking: u16,
    pub raw: [u8; 18],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeLimits {
    pub min_vertical_hz: u8,
    pub max_vertical_hz: u8,
    pub min_horizontal_khz: u8,
    pub max_horizontal_khz: u8,
    pub max_pixel_clock_mhz: u16,
    pub raw: [u8; 18],
}

#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
    /// An empty 18-byte descriptor slot (`00 00 ... 00`).
    Unused,
    DetailedTiming(DetailedTiming),
    MonitorName(String),
    MonitorSerial(String),
    RangeLimits(RangeLimits),
    Unknown([u8; 18]),
}

pub(crate) fn parse_descriptor(bytes: &[u8]) -> Result<Descriptor, EdidError> {
    if bytes.len() != 18 {
        return Err(EdidError::ParseError(
            "descriptor must be 18 bytes".to_owned(),
        ));
    }

    let mut raw = [0_u8; 18];
    raw.copy_from_slice(bytes);

    if raw == [0_u8; 18] {
        return Ok(Descriptor::Unused);
    }

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

pub(crate) fn write_descriptor(descriptor: &Descriptor) -> Result<[u8; 18], EdidError> {
    match descriptor {
        Descriptor::Unused => Ok([0_u8; 18]),
        Descriptor::DetailedTiming(timing) => Ok(timing.raw),
        Descriptor::MonitorName(name) => write_text_descriptor(0xfc, name),
        Descriptor::MonitorSerial(serial) => write_text_descriptor(0xff, serial),
        Descriptor::RangeLimits(range) => Ok(range.raw),
        Descriptor::Unknown(raw) => Ok(*raw),
    }
}

fn write_text_descriptor(tag: u8, text: &str) -> Result<[u8; 18], EdidError> {
    let mut descriptor = [0_u8; 18];
    descriptor[3] = tag;
    let encoded = crate::utils::encode_descriptor_text(text)?;
    descriptor[5..18].copy_from_slice(&encoded);
    Ok(descriptor)
}
