use crate::error::EdidError;

pub(crate) fn validate_checksum(block: &[u8], block_index: usize) -> Result<(), EdidError> {
    if block.len() != 128 {
        return Err(EdidError::InvalidLength);
    }

    let sum = block.iter().fold(0_u8, |acc, byte| acc.wrapping_add(*byte));
    if sum != 0 {
        return Err(EdidError::InvalidChecksum { block_index });
    }
    Ok(())
}

pub(crate) fn fix_checksum(block: &mut [u8]) {
    let sum = block[..127]
        .iter()
        .fold(0_u8, |acc, byte| acc.wrapping_add(*byte));
    block[127] = (0_u8).wrapping_sub(sum);
}

#[cfg(test)]
mod tests {
    use super::{fix_checksum, validate_checksum};

    #[test]
    fn fixes_checksum_for_block() {
        let mut block = [0_u8; 128];
        block[0] = 0x12;
        block[1] = 0x34;

        fix_checksum(&mut block);

        validate_checksum(&block, 0).unwrap();
    }
}
