use crate::{
    error::EdidError,
    model::{
        DisplayIdDataBlock, DisplayIdDataBlockKind, DisplayIdExtension, DisplayIdProductDataBlock,
        DisplayIdVendorSpecificDataBlock,
    },
};

pub(crate) fn parse_displayid_extension(block: &[u8]) -> Result<DisplayIdExtension, EdidError> {
    if block.len() != 128 {
        return Err(EdidError::InvalidLength);
    }

    let payload_bytes = usize::from(block[3]);
    let payload_end = 6 + payload_bytes;
    if payload_end > 127 {
        return Err(EdidError::ParseError(
            "DisplayID payload exceeds extension boundary".to_owned(),
        ));
    }

    let mut data_blocks = Vec::new();
    let mut cursor = 6usize;
    while cursor < payload_end {
        if payload_end - cursor < 3 {
            return Err(EdidError::ParseError(
                "truncated DisplayID data block header".to_owned(),
            ));
        }

        let tag = block[cursor];
        let revision = block[cursor + 1];
        let payload_len = usize::from(block[cursor + 2]);
        let start = cursor + 3;
        let end = start + payload_len;
        if end > payload_end {
            return Err(EdidError::ParseError(
                "DisplayID data block exceeds declared payload size".to_owned(),
            ));
        }

        data_blocks.push(DisplayIdDataBlock {
            tag,
            revision,
            payload: block[start..end].to_vec(),
            kind: parse_displayid_data_block_kind(tag, &block[start..end]),
        });
        cursor = end;
    }

    Ok(DisplayIdExtension {
        version: block[1],
        revision: block[2],
        payload_bytes: block[3],
        product_type: block[4],
        extension_count: block[5],
        data_blocks,
        raw_block: block.to_vec(),
    })
}

fn parse_displayid_data_block_kind(tag: u8, payload: &[u8]) -> DisplayIdDataBlockKind {
    match tag {
        0x20 => DisplayIdDataBlockKind::Product(DisplayIdProductDataBlock {
            payload: payload.to_vec(),
        }),
        0x7f => DisplayIdDataBlockKind::VendorSpecific(DisplayIdVendorSpecificDataBlock {
            payload: payload.to_vec(),
        }),
        _ => DisplayIdDataBlockKind::Unknown,
    }
}
