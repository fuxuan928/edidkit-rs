use crate::{
    model::{Cta861Extension, DataBlock, VendorBlock},
    utils::fix_checksum,
};

pub(crate) fn write_cta861_extension(cta: &Cta861Extension) -> Vec<u8> {
    let Some(data_block_bytes) = serialize_data_blocks(&cta.data_blocks) else {
        return cta.raw_block.clone();
    };

    if data_block_bytes.len() > 123 {
        return cta.raw_block.clone();
    }

    let old_offset = usize::from(cta.detailed_timing_offset).clamp(4, 127);
    let mut trailing = if cta.raw_block.len() == 128 {
        cta.raw_block[old_offset..127].to_vec()
    } else {
        Vec::new()
    };

    let new_offset = 4 + data_block_bytes.len();
    let available_trailing = 127usize.saturating_sub(new_offset);
    if trailing.len() > available_trailing {
        trailing.truncate(available_trailing);
    }

    let mut block = vec![0_u8; 128];
    if cta.raw_block.len() == 128 {
        block.copy_from_slice(&cta.raw_block);
    }

    block[0] = 0x02;
    block[1] = cta.revision;
    block[2] = new_offset as u8;
    block[3] = cta.flags;
    block[4..new_offset].copy_from_slice(&data_block_bytes);

    let trailing_end = new_offset + trailing.len();
    block[new_offset..trailing_end].copy_from_slice(&trailing);
    for byte in &mut block[trailing_end..127] {
        *byte = 0;
    }

    fix_checksum(&mut block);
    block
}

fn serialize_data_blocks(blocks: &[DataBlock]) -> Option<Vec<u8>> {
    let mut out = Vec::new();

    for block in blocks {
        let (tag, payload) = serialize_data_block(block)?;
        if payload.len() > 0x1f {
            return None;
        }

        out.push((tag << 5) | payload.len() as u8);
        out.extend_from_slice(&payload);
    }

    Some(out)
}

fn serialize_data_block(block: &DataBlock) -> Option<(u8, Vec<u8>)> {
    match block {
        DataBlock::Video(video) => Some((0x02, video.vic_codes.clone())),
        DataBlock::Audio(audio) => {
            let payload = audio
                .short_audio_descriptors
                .iter()
                .flat_map(|descriptor| descriptor.iter().copied())
                .collect();
            Some((0x01, payload))
        }
        DataBlock::Vendor(vendor) => Some((0x03, serialize_vendor_block(vendor))),
        DataBlock::SpeakerAllocation(block) => Some((0x04, block.bytes.clone())),
        DataBlock::HdrStaticMetadata(block) => {
            let mut payload = vec![
                0x06,
                block.electro_optical_transfer_functions,
                block.static_metadata_descriptors,
            ];
            if let Some(value) = block.desired_content_max_luminance {
                payload.push(value);
            }
            if let Some(value) = block.desired_content_max_frame_average_luminance {
                payload.push(value);
            }
            if let Some(value) = block.desired_content_min_luminance {
                payload.push(value);
            }
            Some((0x07, payload))
        }
        DataBlock::Extended(block) => {
            let mut payload = Vec::with_capacity(1 + block.payload.len());
            payload.push(block.extended_tag);
            payload.extend_from_slice(&block.payload);
            Some((0x07, payload))
        }
        DataBlock::Unknown { tag, payload } if *tag <= 0x07 => Some((*tag, payload.clone())),
        DataBlock::Unknown { .. } => None,
    }
}

fn serialize_vendor_block(vendor: &VendorBlock) -> Vec<u8> {
    let mut payload = Vec::with_capacity(3 + vendor.payload.len());
    payload.extend_from_slice(&vendor.oui);
    payload.extend_from_slice(&vendor.payload);
    payload
}
