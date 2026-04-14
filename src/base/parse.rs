use crate::{
    base::{
        AnalogVideoInput, BaseBlock, DigitalVideoInput, EdidVersion, ManufactureDate,
        ManufacturerId, VideoInputDefinition, descriptor::parse_descriptor,
    },
    error::EdidError,
};

pub(crate) fn parse_base_block(block: &[u8]) -> Result<BaseBlock, EdidError> {
    if block.len() != 128 {
        return Err(EdidError::InvalidLength);
    }

    crate::utils::validate_checksum(block, 0)?;

    let mut raw_block = [0_u8; 128];
    raw_block.copy_from_slice(block);

    let manufacturer_id =
        ManufacturerId(crate::utils::decode_manufacturer_id([block[8], block[9]])?);
    let product_code = u16::from_le_bytes([block[10], block[11]]);
    let serial_number = u32::from_le_bytes([block[12], block[13], block[14], block[15]]);
    let manufacture_date = ManufactureDate {
        week: block[16],
        year: 1990 + u16::from(block[17]),
    };
    let version = EdidVersion {
        major: block[18],
        minor: block[19],
    };
    let video_input_definition = parse_video_input_definition(&version, block[20]);

    let mut descriptors = Vec::with_capacity(4);
    for chunk in block[54..126].chunks_exact(18) {
        descriptors.push(parse_descriptor(chunk)?);
    }
    let descriptors = descriptors
        .try_into()
        .expect("base block must contain exactly four descriptors");

    Ok(BaseBlock {
        manufacturer_id,
        product_code,
        serial_number,
        manufacture_date,
        version,
        video_input_definition,
        extension_count: block[126],
        descriptors,
        raw_block,
    })
}

fn parse_video_input_definition(version: &EdidVersion, value: u8) -> VideoInputDefinition {
    if (value & 0x80) == 0 {
        return VideoInputDefinition::Analog(AnalogVideoInput {
            separate_sync_supported: (value & 0x08) != 0,
            composite_sync_on_hsync_supported: (value & 0x04) != 0,
            composite_sync_on_green_supported: (value & 0x02) != 0,
            serration_supported: (value & 0x01) != 0,
        });
    }

    if version.major > 1 || (version.major == 1 && version.minor >= 4) {
        let color_bit_depth = match (value >> 4) & 0x07 {
            0 => None,
            1 => Some(6),
            2 => Some(8),
            3 => Some(10),
            4 => Some(12),
            5 => Some(14),
            6 => Some(16),
            _ => None,
        };

        VideoInputDefinition::Digital(DigitalVideoInput {
            dfp_1x_compatible: false,
            color_bit_depth,
            interface: match value & 0x0f {
                0 => None,
                n => Some(n),
            },
        })
    } else {
        VideoInputDefinition::Digital(DigitalVideoInput {
            dfp_1x_compatible: (value & 0x01) != 0,
            color_bit_depth: None,
            interface: None,
        })
    }
}
