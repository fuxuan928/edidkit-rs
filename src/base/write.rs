use crate::{
    base::{BaseBlock, descriptor::write_descriptor},
    utils::{encode_manufacturer_id, fix_checksum},
};

pub(crate) fn write_base_block(base: &BaseBlock) -> [u8; 128] {
    let mut block = base.raw_block;

    if let Ok(manufacturer_id) = encode_manufacturer_id(&base.manufacturer_id.0) {
        block[8..10].copy_from_slice(&manufacturer_id);
    }
    block[10..12].copy_from_slice(&base.product_code.to_le_bytes());
    block[12..16].copy_from_slice(&base.serial_number.to_le_bytes());
    block[16] = base.manufacture_date.week;
    block[17] = base.manufacture_date.year.saturating_sub(1990) as u8;
    block[18] = base.version.major;
    block[19] = base.version.minor;
    block[20] = write_video_input_definition(base);
    block[126] = base.extension_count;

    for (index, descriptor) in base.descriptors.iter().enumerate().take(4) {
        let start = 54 + index * 18;
        block[start..start + 18].copy_from_slice(&write_descriptor(descriptor));
    }

    fix_checksum(&mut block);
    block
}

fn write_video_input_definition(base: &BaseBlock) -> u8 {
    match &base.video_input_definition {
        crate::base::VideoInputDefinition::Analog(input) => {
            (u8::from(input.separate_sync_supported) << 3)
                | (u8::from(input.composite_sync_on_hsync_supported) << 2)
                | (u8::from(input.composite_sync_on_green_supported) << 1)
                | u8::from(input.serration_supported)
        }
        crate::base::VideoInputDefinition::Digital(input) => {
            let mut value = 0x80;
            if base.version.major > 1 || (base.version.major == 1 && base.version.minor >= 4) {
                value |= match input.color_bit_depth {
                    Some(6) => 1 << 4,
                    Some(8) => 2 << 4,
                    Some(10) => 3 << 4,
                    Some(12) => 4 << 4,
                    Some(14) => 5 << 4,
                    Some(16) => 6 << 4,
                    _ => 0,
                };
                value |= input.interface.unwrap_or(0) & 0x0f;
            } else if input.dfp_1x_compatible {
                value |= 0x01;
            }
            value
        }
    }
}
