mod bitfield;
mod checksum;

pub(crate) use bitfield::BitReader;
pub(crate) use checksum::{fix_checksum, validate_checksum};

use crate::error::EdidError;

const EDID_HEADER: [u8; 8] = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];

pub(crate) fn validate_header(data: &[u8]) -> Result<(), EdidError> {
    if data.len() < 8 || data[..8] != EDID_HEADER {
        return Err(EdidError::InvalidHeader);
    }

    Ok(())
}

pub(crate) fn decode_manufacturer_id(bytes: [u8; 2]) -> Result<String, EdidError> {
    let value = u16::from_be_bytes(bytes);
    let chars = [
        ((value >> 10) & 0x1f) as u8,
        ((value >> 5) & 0x1f) as u8,
        (value & 0x1f) as u8,
    ];

    let mut out = String::with_capacity(3);
    for code in chars {
        if !(1..=26).contains(&code) {
            return Err(EdidError::ParseError(
                "invalid manufacturer id encoding".to_owned(),
            ));
        }
        out.push(char::from(b'A' + code - 1));
    }

    Ok(out)
}

pub(crate) fn encode_manufacturer_id(id: &str) -> Result<[u8; 2], EdidError> {
    let bytes = id.as_bytes();
    if bytes.len() != 3 || !bytes.iter().all(|b| b.is_ascii_uppercase()) {
        return Err(EdidError::ValidationError(
            "manufacturer id must be three uppercase ASCII letters".to_owned(),
        ));
    }

    let value = (u16::from(bytes[0] - b'A' + 1) << 10)
        | (u16::from(bytes[1] - b'A' + 1) << 5)
        | u16::from(bytes[2] - b'A' + 1);
    Ok(value.to_be_bytes())
}

pub(crate) fn parse_descriptor_text(bytes: &[u8]) -> String {
    let end = bytes
        .iter()
        .position(|byte| matches!(byte, 0x0a | 0x00))
        .unwrap_or(bytes.len());
    let text = &bytes[..end];
    String::from_utf8_lossy(text).trim_end().to_owned()
}

pub(crate) fn validate_descriptor_text(text: &str) -> Result<(), EdidError> {
    if !text.is_ascii() {
        return Err(EdidError::ValidationError(
            "descriptor text must be ASCII".to_owned(),
        ));
    }
    if text.len() > 13 {
        return Err(EdidError::ValidationError(
            "descriptor text cannot exceed 13 characters".to_owned(),
        ));
    }
    Ok(())
}

pub(crate) fn encode_descriptor_text(text: &str) -> Result<[u8; 13], EdidError> {
    validate_descriptor_text(text)?;

    let mut out = [b' '; 13];
    let bytes = text.as_bytes();
    out[..bytes.len()].copy_from_slice(bytes);
    if bytes.len() < 13 {
        out[bytes.len()] = 0x0a;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{
        decode_manufacturer_id, encode_descriptor_text, encode_manufacturer_id,
        parse_descriptor_text, validate_descriptor_text,
    };

    #[test]
    fn manufacturer_id_round_trip() {
        let encoded = encode_manufacturer_id("DEL").unwrap();
        assert_eq!(decode_manufacturer_id(encoded).unwrap(), "DEL");
    }

    #[test]
    fn rejects_invalid_manufacturer_id() {
        assert!(encode_manufacturer_id("de1").is_err());
        assert!(decode_manufacturer_id([0x00, 0x00]).is_err());
    }

    #[test]
    fn descriptor_text_round_trip() {
        let encoded = encode_descriptor_text("RK-UHD").unwrap();
        assert_eq!(parse_descriptor_text(&encoded), "RK-UHD");
    }

    #[test]
    fn descriptor_text_validation_rejects_invalid_input() {
        assert!(validate_descriptor_text("0123456789ABCD").is_err());
        assert!(validate_descriptor_text("显示器").is_err());
    }
}
