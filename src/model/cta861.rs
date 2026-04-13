use crate::EdidError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoBlock {
    pub vic_codes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioBlock {
    pub short_audio_descriptors: Vec<[u8; 3]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorBlock {
    pub oui: [u8; 3],
    pub payload: Vec<u8>,
    pub hdmi: Option<HdmiVendorBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdmiVendorBlock {
    pub physical_address: [u8; 4],
    pub supports_ai: bool,
    pub deep_color_48bit: bool,
    pub deep_color_36bit: bool,
    pub deep_color_30bit: bool,
    pub deep_color_y444: bool,
    pub dvi_dual_link: bool,
    pub max_tmds_clock_mhz: Option<u16>,
    pub latency_fields_present: bool,
    pub interlaced_latency_fields_present: bool,
    pub hdmi_video_present: bool,
    pub cnc_graphics: bool,
    pub cnc_photo: bool,
    pub cnc_cinema: bool,
    pub cnc_game: bool,
    pub video_latency: Option<u8>,
    pub audio_latency: Option<u8>,
    pub interlaced_video_latency: Option<u8>,
    pub interlaced_audio_latency: Option<u8>,
}

impl VendorBlock {
    pub fn set_hdmi_max_tmds_clock_mhz(&mut self, mhz: u16) -> Result<(), EdidError> {
        if self.oui != [0x03, 0x0c, 0x00] {
            return Err(EdidError::ValidationError(
                "vendor block is not an HDMI VSDB".to_owned(),
            ));
        }
        if mhz == 0 || mhz % 5 != 0 || mhz > u16::from(u8::MAX) * 5 {
            return Err(EdidError::ValidationError(
                "HDMI max TMDS clock must be in 5 MHz units between 5 and 1275".to_owned(),
            ));
        }

        while self.payload.len() < 4 {
            self.payload.push(0);
        }
        self.payload[3] = (mhz / 5) as u8;

        if let Some(hdmi) = &mut self.hdmi {
            hdmi.max_tmds_clock_mhz = Some(mhz);
        }

        Ok(())
    }

    pub fn set_hdmi_content_types(
        &mut self,
        graphics: bool,
        photo: bool,
        cinema: bool,
        game: bool,
    ) -> Result<(), EdidError> {
        if self.oui != [0x03, 0x0c, 0x00] {
            return Err(EdidError::ValidationError(
                "vendor block is not an HDMI VSDB".to_owned(),
            ));
        }

        while self.payload.len() < 6 {
            self.payload.push(0);
        }
        self.payload[4] |= 0x20;
        self.payload[5] = (u8::from(graphics) << 0)
            | (u8::from(photo) << 1)
            | (u8::from(cinema) << 2)
            | (u8::from(game) << 3);

        if let Some(hdmi) = &mut self.hdmi {
            hdmi.hdmi_video_present = true;
            hdmi.cnc_graphics = graphics;
            hdmi.cnc_photo = photo;
            hdmi.cnc_cinema = cinema;
            hdmi.cnc_game = game;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeakerAllocationBlock {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedTagBlock {
    pub extended_tag: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdrStaticMetadataBlock {
    pub electro_optical_transfer_functions: u8,
    pub static_metadata_descriptors: u8,
    pub desired_content_max_luminance: Option<u8>,
    pub desired_content_max_frame_average_luminance: Option<u8>,
    pub desired_content_min_luminance: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataBlock {
    Video(VideoBlock),
    Audio(AudioBlock),
    Vendor(VendorBlock),
    SpeakerAllocation(SpeakerAllocationBlock),
    HdrStaticMetadata(HdrStaticMetadataBlock),
    Extended(ExtendedTagBlock),
    Unknown { tag: u8, payload: Vec<u8> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cta861Extension {
    pub revision: u8,
    pub detailed_timing_offset: u8,
    pub flags: u8,
    pub data_blocks: Vec<DataBlock>,
    pub raw_block: Vec<u8>,
}

impl Cta861Extension {
    pub fn add_video_vic(&mut self, vic: u8) -> bool {
        if let Some(video) = self.video_block_mut() {
            if video.vic_codes.contains(&vic) {
                return false;
            }
            video.vic_codes.push(vic);
            return true;
        }

        self.data_blocks.insert(
            0,
            DataBlock::Video(VideoBlock {
                vic_codes: vec![vic],
            }),
        );
        true
    }

    pub fn remove_video_vic(&mut self, vic: u8) -> bool {
        let Some(video) = self.video_block_mut() else {
            return false;
        };

        let original_len = video.vic_codes.len();
        video.vic_codes.retain(|existing| *existing != vic);
        original_len != video.vic_codes.len()
    }

    pub fn set_speaker_allocation(&mut self, bytes: &[u8]) {
        if let Some(block) = self.speaker_allocation_block_mut() {
            block.bytes = bytes.to_vec();
            return;
        }

        self.data_blocks
            .push(DataBlock::SpeakerAllocation(SpeakerAllocationBlock {
                bytes: bytes.to_vec(),
            }));
    }

    pub fn set_hdmi_max_tmds_clock_mhz(&mut self, mhz: u16) -> Result<(), EdidError> {
        for block in &mut self.data_blocks {
            if let DataBlock::Vendor(vendor) = block {
                if vendor.oui == [0x03, 0x0c, 0x00] {
                    return vendor.set_hdmi_max_tmds_clock_mhz(mhz);
                }
            }
        }

        Err(EdidError::ValidationError(
            "HDMI vendor-specific block not present".to_owned(),
        ))
    }

    pub fn set_hdmi_content_types(
        &mut self,
        graphics: bool,
        photo: bool,
        cinema: bool,
        game: bool,
    ) -> Result<(), EdidError> {
        for block in &mut self.data_blocks {
            if let DataBlock::Vendor(vendor) = block {
                if vendor.oui == [0x03, 0x0c, 0x00] {
                    return vendor.set_hdmi_content_types(graphics, photo, cinema, game);
                }
            }
        }

        Err(EdidError::ValidationError(
            "HDMI vendor-specific block not present".to_owned(),
        ))
    }

    fn video_block_mut(&mut self) -> Option<&mut VideoBlock> {
        self.data_blocks.iter_mut().find_map(|block| match block {
            DataBlock::Video(video) => Some(video),
            _ => None,
        })
    }

    fn speaker_allocation_block_mut(&mut self) -> Option<&mut SpeakerAllocationBlock> {
        self.data_blocks.iter_mut().find_map(|block| match block {
            DataBlock::SpeakerAllocation(speakers) => Some(speakers),
            _ => None,
        })
    }
}
