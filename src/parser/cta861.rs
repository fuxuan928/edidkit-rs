use crate::{
    error::EdidError,
    model::{
        AudioBlock, Cta861Extension, DataBlock, ExtendedTagBlock, HdrStaticMetadataBlock,
        SpeakerAllocationBlock, VendorBlock, VideoBlock,
    },
};

pub(crate) fn parse_cta861_extension(block: &[u8]) -> Result<Cta861Extension, EdidError> {
    if block.len() != 128 {
        return Err(EdidError::InvalidLength);
    }

    let offset = usize::from(block[2]);
    if !(4..=127).contains(&offset) {
        return Err(EdidError::ParseError(format!(
            "invalid CTA detailed timing offset {offset}"
        )));
    }

    let mut data_blocks = Vec::new();
    let mut cursor = 4usize;
    while cursor < offset {
        let header = block[cursor];
        let tag = header >> 5;
        let len = usize::from(header & 0x1f);
        let end = cursor + 1 + len;
        if end > offset {
            return Err(EdidError::ParseError(
                "CTA data block overruns payload".to_owned(),
            ));
        }

        let payload = &block[cursor + 1..end];
        let data_block = match tag {
            0x01 => parse_audio_block(payload)?,
            0x02 => DataBlock::Video(VideoBlock {
                vic_codes: payload.to_vec(),
            }),
            0x03 => parse_vendor_block(payload)?,
            0x04 => DataBlock::SpeakerAllocation(SpeakerAllocationBlock {
                bytes: payload.to_vec(),
            }),
            0x07 => parse_extended_tag_block(payload)?,
            _ => DataBlock::Unknown {
                tag,
                payload: payload.to_vec(),
            },
        };
        data_blocks.push(data_block);
        cursor = end;
    }

    Ok(Cta861Extension {
        revision: block[1],
        detailed_timing_offset: block[2],
        flags: block[3],
        data_blocks,
        raw_block: block.to_vec(),
    })
}

fn parse_audio_block(payload: &[u8]) -> Result<DataBlock, EdidError> {
    if payload.len() % 3 != 0 {
        return Err(EdidError::ParseError(
            "audio data block length must be divisible by 3".to_owned(),
        ));
    }

    let descriptors = payload
        .chunks_exact(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect();

    Ok(DataBlock::Audio(AudioBlock {
        short_audio_descriptors: descriptors,
    }))
}

fn parse_vendor_block(payload: &[u8]) -> Result<DataBlock, EdidError> {
    if payload.len() < 3 {
        return Err(EdidError::ParseError(
            "vendor block must contain at least 3 bytes for OUI".to_owned(),
        ));
    }

    Ok(DataBlock::Vendor(VendorBlock {
        oui: [payload[0], payload[1], payload[2]],
        payload: payload[3..].to_vec(),
        hdmi: parse_hdmi_vendor_block([payload[0], payload[1], payload[2]], &payload[3..]),
    }))
}

fn parse_extended_tag_block(payload: &[u8]) -> Result<DataBlock, EdidError> {
    if payload.is_empty() {
        return Err(EdidError::ParseError(
            "extended tag block must contain an extended tag".to_owned(),
        ));
    }

    if payload[0] == 0x06 {
        return parse_hdr_static_metadata_block(&payload[1..]);
    }

    Ok(DataBlock::Extended(ExtendedTagBlock {
        extended_tag: payload[0],
        payload: payload[1..].to_vec(),
    }))
}

fn parse_hdr_static_metadata_block(payload: &[u8]) -> Result<DataBlock, EdidError> {
    if payload.len() < 2 {
        return Err(EdidError::ParseError(
            "HDR static metadata block must contain at least two bytes".to_owned(),
        ));
    }

    Ok(DataBlock::HdrStaticMetadata(HdrStaticMetadataBlock {
        electro_optical_transfer_functions: payload[0],
        static_metadata_descriptors: payload[1],
        desired_content_max_luminance: payload.get(2).copied(),
        desired_content_max_frame_average_luminance: payload.get(3).copied(),
        desired_content_min_luminance: payload.get(4).copied(),
    }))
}

fn parse_hdmi_vendor_block(oui: [u8; 3], payload: &[u8]) -> Option<crate::model::HdmiVendorBlock> {
    if oui != [0x03, 0x0c, 0x00] || payload.len() < 3 {
        return None;
    }

    let feature_flags = payload[2];
    let capabilities = payload.get(4).copied().unwrap_or(0);
    let content_types = payload.get(5).copied().unwrap_or(0);
    let mut next_index = 6usize;
    let latency_fields_present = (capabilities & 0x80) != 0;
    let interlaced_latency_fields_present = (capabilities & 0x40) != 0;
    let hdmi_video_present = (capabilities & 0x20) != 0;

    let video_latency = if latency_fields_present && payload.len() > next_index {
        let value = payload[next_index];
        next_index += 1;
        Some(value)
    } else {
        None
    };
    let audio_latency = if latency_fields_present && payload.len() > next_index {
        let value = payload[next_index];
        next_index += 1;
        Some(value)
    } else {
        None
    };
    let interlaced_video_latency =
        if interlaced_latency_fields_present && payload.len() > next_index {
            let value = payload[next_index];
            next_index += 1;
            Some(value)
        } else {
            None
        };
    let interlaced_audio_latency =
        if interlaced_latency_fields_present && payload.len() > next_index {
            Some(payload[next_index])
        } else {
            None
        };

    Some(crate::model::HdmiVendorBlock {
        physical_address: [
            payload[0] >> 4,
            payload[0] & 0x0f,
            payload[1] >> 4,
            payload[1] & 0x0f,
        ],
        supports_ai: (feature_flags & 0x80) != 0,
        deep_color_48bit: (feature_flags & 0x40) != 0,
        deep_color_36bit: (feature_flags & 0x20) != 0,
        deep_color_30bit: (feature_flags & 0x10) != 0,
        deep_color_y444: (feature_flags & 0x08) != 0,
        dvi_dual_link: (feature_flags & 0x01) != 0,
        max_tmds_clock_mhz: payload.get(3).map(|value| u16::from(*value) * 5),
        latency_fields_present,
        interlaced_latency_fields_present,
        hdmi_video_present,
        cnc_graphics: (content_types & 0x01) != 0,
        cnc_photo: (content_types & 0x02) != 0,
        cnc_cinema: (content_types & 0x04) != 0,
        cnc_game: (content_types & 0x08) != 0,
        video_latency,
        audio_latency,
        interlaced_video_latency,
        interlaced_audio_latency,
    })
}
