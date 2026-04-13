use edidkit::{DataBlock, Descriptor, Edid, EdidError, ExtensionBlock};

const EDID_FORCE_GBR24: [u8; 256] = [
    0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x49, 0x70, 0x88, 0x35, 0x01, 0x00, 0x00, 0x00,
    0x2d, 0x1f, 0x01, 0x03, 0x80, 0x78, 0x44, 0x78, 0x0a, 0xcf, 0x74, 0xa3, 0x57, 0x4c, 0xb0, 0x23,
    0x09, 0x48, 0x4c, 0x21, 0x08, 0x00, 0x61, 0x40, 0x01, 0x01, 0x81, 0x00, 0x95, 0x00, 0xa9, 0xc0,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x3a, 0x80, 0x18, 0x71, 0x38, 0x2d, 0x40, 0x58, 0x2c,
    0x45, 0x00, 0x20, 0xc2, 0x31, 0x00, 0x00, 0x1e, 0x01, 0x1d, 0x00, 0x72, 0x51, 0xd0, 0x1e, 0x20,
    0x6e, 0x28, 0x55, 0x00, 0x20, 0xc2, 0x31, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0xfc, 0x00, 0x52,
    0x4b, 0x2d, 0x55, 0x48, 0x44, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xfd,
    0x00, 0x3b, 0x46, 0x1f, 0x8c, 0x3c, 0x00, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x01, 0xa7,
    0x02, 0x03, 0x2a, 0xc1, 0x51, 0x07, 0x16, 0x14, 0x05, 0x01, 0x03, 0x12, 0x13, 0x84, 0x22, 0x1f,
    0x90, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x23, 0x09, 0x07, 0x07, 0x83, 0x01, 0x00, 0x00, 0x67, 0x03,
    0x0c, 0x00, 0x30, 0x00, 0x00, 0x44, 0xe3, 0x05, 0x03, 0x01, 0x02, 0x3a, 0x80, 0x18, 0x71, 0x38,
    0x2d, 0x40, 0x58, 0x2c, 0x45, 0x00, 0x20, 0xc2, 0x31, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8,
];

const EDID_FORCE_GBR24_60HZ: [u8; 256] = [
    0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x49, 0x70, 0x88, 0x35, 0x01, 0x00, 0x00, 0x00,
    0x2d, 0x1f, 0x01, 0x03, 0x80, 0x78, 0x44, 0x78, 0x0a, 0xcf, 0x74, 0xa3, 0x57, 0x4c, 0xb0, 0x23,
    0x09, 0x48, 0x4c, 0x21, 0x08, 0x00, 0x61, 0x40, 0x01, 0x01, 0x81, 0x00, 0x95, 0x00, 0xa9, 0xc0,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x3a, 0x80, 0x18, 0x71, 0x38, 0x2d, 0x40, 0x58, 0x2c,
    0x45, 0x00, 0x20, 0xc2, 0x31, 0x00, 0x00, 0x1e, 0x01, 0x1d, 0x00, 0x72, 0x51, 0xd0, 0x1e, 0x20,
    0x6e, 0x28, 0x55, 0x00, 0x20, 0xc2, 0x31, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0xfc, 0x00, 0x52,
    0x4b, 0x2d, 0x55, 0x48, 0x44, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xfd,
    0x00, 0x3b, 0x46, 0x1f, 0x8c, 0x3c, 0x00, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x01, 0xa7,
    0x02, 0x03, 0x20, 0xc1, 0x47, 0x07, 0x05, 0x01, 0x03, 0x84, 0x90, 0x61, 0x23, 0x09, 0x07, 0x07,
    0x83, 0x01, 0x00, 0x00, 0x67, 0x03, 0x0c, 0x00, 0x30, 0x00, 0x00, 0x44, 0xe3, 0x05, 0x03, 0x01,
    0x02, 0x3a, 0x80, 0x18, 0x71, 0x38, 0x2d, 0x40, 0x58, 0x2c, 0x45, 0x00, 0x20, 0xc2, 0x31, 0x00,
    0x00, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd6,
];

const EDID_600M: [u8; 256] = [
    0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x49, 0x70, 0x88, 0x35, 0x01, 0x00, 0x00, 0x00,
    0x2d, 0x1f, 0x01, 0x03, 0x80, 0x78, 0x44, 0x78, 0x0a, 0xcf, 0x74, 0xa3, 0x57, 0x4c, 0xb0, 0x23,
    0x09, 0x48, 0x4c, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x08, 0xe8, 0x00, 0x30, 0xf2, 0x70, 0x5a, 0x80, 0xb0, 0x58,
    0x8a, 0x00, 0xc4, 0x8e, 0x21, 0x00, 0x00, 0x1e, 0x08, 0xe8, 0x00, 0x30, 0xf2, 0x70, 0x5a, 0x80,
    0xb0, 0x58, 0x8a, 0x00, 0x20, 0xc2, 0x31, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0xfc, 0x00, 0x52,
    0x4b, 0x2d, 0x55, 0x48, 0x44, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xfd,
    0x00, 0x3b, 0x46, 0x1f, 0x8c, 0x3c, 0x00, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x01, 0x39,
    0x02, 0x03, 0x22, 0xf2, 0x41, 0x61, 0x23, 0x09, 0x07, 0x07, 0x83, 0x01, 0x00, 0x00, 0x67, 0x03,
    0x0c, 0x00, 0x30, 0x00, 0x00, 0x78, 0x67, 0xd8, 0x5d, 0xc4, 0x01, 0x78, 0xc0, 0x00, 0xe3, 0x05,
    0x03, 0x01, 0x08, 0xe8, 0x00, 0x30, 0xf2, 0x70, 0x5a, 0x80, 0xb0, 0x58, 0x8a, 0x00, 0xc4, 0x8e,
    0x21, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65,
];

const EDID_600M_U2418: [u8; 128] = [
    0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x05, 0xe3, 0x18, 0x24, 0x01, 0x00, 0x00, 0x00,
    0x30, 0x1b, 0x01, 0x03, 0x80, 0x78, 0x44, 0x78, 0x0a, 0xcf, 0x74, 0xa3, 0x57, 0x4c, 0xb0, 0x23,
    0x09, 0x48, 0x4c, 0x21, 0x08, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x08, 0xe8, 0x00, 0x30, 0xf2, 0x70, 0x5a, 0x80, 0xb0, 0x58,
    0x8a, 0x00, 0xc4, 0x8e, 0x21, 0x00, 0x00, 0x1e, 0x08, 0xe8, 0x00, 0x30, 0xf2, 0x70, 0x5a, 0x80,
    0xb0, 0x58, 0x8a, 0x00, 0x20, 0xc2, 0x31, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4,
];

#[test]
fn parses_base_only_sample() {
    let edid = Edid::parse(&EDID_600M_U2418).unwrap();

    assert_eq!(edid.base.manufacturer_id.0, "AOC");
    assert_eq!(edid.base.product_code, 0x2418);
    assert_eq!(edid.base.extension_count, 0);
    assert_eq!(edid.extensions.len(), 0);
    assert!(matches!(
        edid.base.video_input_definition,
        edidkit::VideoInputDefinition::Digital(_)
    ));
    assert!(matches!(
        edid.base.descriptors[0],
        Descriptor::DetailedTiming(_)
    ));
}

#[test]
fn parses_version_aware_analog_video_input() {
    let mut sample = EDID_600M_U2418;
    sample[20] = 0x0f;
    fix_checksum(&mut sample);

    let edid = Edid::parse(&sample).unwrap();
    let edidkit::VideoInputDefinition::Analog(video_input) = edid.base.video_input_definition
    else {
        panic!("expected analog video input");
    };

    assert!(video_input.separate_sync_supported);
    assert!(video_input.composite_sync_on_hsync_supported);
    assert!(video_input.composite_sync_on_green_supported);
    assert!(video_input.serration_supported);
}

#[test]
fn parses_displayid_extension() {
    let bytes = make_displayid_sample();
    let edid = Edid::parse(&bytes).unwrap();

    assert_eq!(edid.extensions.len(), 1);
    let ExtensionBlock::DisplayId(display_id) = &edid.extensions[0] else {
        panic!("expected DisplayID extension");
    };

    assert_eq!(display_id.version, 0x01);
    assert_eq!(display_id.revision, 0x03);
    assert_eq!(display_id.payload_bytes, 0x0b);
    assert_eq!(display_id.product_type, 0x02);
    assert_eq!(display_id.extension_count, 0x00);
    assert_eq!(display_id.data_blocks.len(), 2);
    assert_eq!(display_id.data_blocks[0].tag, 0x20);
    assert_eq!(display_id.data_blocks[0].revision, 0x01);
    assert_eq!(display_id.data_blocks[0].payload, vec![0x12, 0x34, 0x56]);
    assert!(matches!(
        display_id.data_blocks[0].kind,
        edidkit::DisplayIdDataBlockKind::Product(_)
    ));
    assert_eq!(display_id.data_blocks[1].tag, 0x7f);
    assert_eq!(display_id.data_blocks[1].revision, 0x00);
    assert_eq!(display_id.data_blocks[1].payload, vec![0xaa, 0xbb]);
    assert!(matches!(
        display_id.data_blocks[1].kind,
        edidkit::DisplayIdDataBlockKind::VendorSpecific(_)
    ));
}

#[test]
fn round_trip_preserves_all_sample_bytes() {
    for sample in [
        EDID_600M_U2418.as_slice(),
        EDID_FORCE_GBR24.as_slice(),
        EDID_FORCE_GBR24_60HZ.as_slice(),
        EDID_600M.as_slice(),
        make_displayid_sample().as_slice(),
    ] {
        let edid = Edid::parse(sample).unwrap();
        assert_eq!(edid.to_bytes(), sample);
    }
}

#[test]
fn parses_cta_extension_blocks() {
    let edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();

    assert_eq!(edid.extensions.len(), 1);
    let ExtensionBlock::Cta861(cta) = &edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    assert_eq!(cta.revision, 3);
    assert_eq!(cta.detailed_timing_offset, 42);
    assert!(matches!(cta.data_blocks[0], DataBlock::Video(_)));
    assert!(matches!(cta.data_blocks[1], DataBlock::Audio(_)));
    assert!(matches!(
        cta.data_blocks[2],
        DataBlock::SpeakerAllocation(_)
    ));
    assert!(matches!(cta.data_blocks[3], DataBlock::Vendor(_)));
    assert!(matches!(cta.data_blocks[4], DataBlock::Extended(_)));
}

#[test]
fn parses_typed_speaker_and_extended_blocks() {
    let edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let DataBlock::SpeakerAllocation(speakers) = &cta.data_blocks[2] else {
        panic!("expected speaker allocation block");
    };
    assert_eq!(speakers.bytes, vec![0x01, 0x00, 0x00]);

    let DataBlock::Extended(block) = &cta.data_blocks[4] else {
        panic!("expected extended tag block");
    };
    assert_eq!(block.extended_tag, 0x05);
    assert_eq!(block.payload, vec![0x03, 0x01]);
}

#[test]
fn parses_hdr_static_metadata_block() {
    let bytes = make_hdr_static_metadata_sample();
    let edid = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let hdr_block = cta.data_blocks.iter().find_map(|block| match block {
        DataBlock::HdrStaticMetadata(block) => Some(block),
        _ => None,
    });

    let hdr_block = hdr_block.expect("expected HDR static metadata block");
    assert_eq!(hdr_block.electro_optical_transfer_functions, 0x05);
    assert_eq!(hdr_block.static_metadata_descriptors, 0x01);
    assert_eq!(hdr_block.desired_content_max_luminance, Some(0x64));
    assert_eq!(
        hdr_block.desired_content_max_frame_average_luminance,
        Some(0x32)
    );
    assert_eq!(hdr_block.desired_content_min_luminance, Some(0x0a));
}

#[test]
fn parses_multiple_vendor_blocks_in_cta() {
    let edid = Edid::parse(&EDID_600M).unwrap();
    let ExtensionBlock::Cta861(cta) = &edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let vendor_count = cta
        .data_blocks
        .iter()
        .filter(|block| matches!(block, DataBlock::Vendor(_)))
        .count();
    assert_eq!(vendor_count, 2);
}

#[test]
fn parses_hdmi_vendor_specific_block_details() {
    let edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let DataBlock::Vendor(vendor) = &cta.data_blocks[3] else {
        panic!("expected vendor block");
    };

    assert_eq!(vendor.oui, [0x03, 0x0c, 0x00]);
    let hdmi = vendor.hdmi.as_ref().expect("expected HDMI vendor block");
    assert_eq!(hdmi.physical_address, [3, 0, 0, 0]);
    assert_eq!(hdmi.max_tmds_clock_mhz, Some(340));
    assert!(!hdmi.supports_ai);
    assert!(!hdmi.deep_color_30bit);
    assert!(!hdmi.hdmi_video_present);
}

#[test]
fn keeps_non_hdmi_vendor_block_generic() {
    let edid = Edid::parse(&EDID_600M).unwrap();
    let ExtensionBlock::Cta861(cta) = &edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let vendor_blocks: Vec<_> = cta
        .data_blocks
        .iter()
        .filter_map(|block| match block {
            DataBlock::Vendor(vendor) => Some(vendor),
            _ => None,
        })
        .collect();

    assert_eq!(vendor_blocks.len(), 2);
    assert!(vendor_blocks[0].hdmi.is_some());
    assert_eq!(
        vendor_blocks[0].hdmi.as_ref().unwrap().max_tmds_clock_mhz,
        Some(600)
    );
    assert!(vendor_blocks[1].hdmi.is_none());
}

#[test]
fn updates_product_code_and_monitor_name() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();

    edid.set_product_code(0x4321);
    edid.set_monitor_name("RK-UHD-ALT").unwrap();

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();

    assert_eq!(reparsed.base.product_code, 0x4321);
    assert!(reparsed.base.descriptors.iter().any(
        |descriptor| matches!(descriptor, Descriptor::MonitorName(name) if name == "RK-UHD-ALT")
    ));
}

#[test]
fn rewrites_cta_extension_from_typed_model_without_changes() {
    let edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let bytes = edid.to_bytes();

    assert_eq!(bytes, EDID_FORCE_GBR24);
}

#[test]
fn serializes_cta_video_block_mutation() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24_60HZ).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let DataBlock::Video(video) = &mut cta.data_blocks[0] else {
        panic!("expected video block");
    };
    video.vic_codes[0] = 0x10;

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &reparsed.extensions[0] else {
        panic!("expected CTA-861 extension");
    };
    let DataBlock::Video(video) = &cta.data_blocks[0] else {
        panic!("expected video block");
    };

    assert_eq!(video.vic_codes[0], 0x10);
}

#[test]
fn serializes_cta_extended_block_mutation() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let DataBlock::Extended(block) = &mut cta.data_blocks[4] else {
        panic!("expected extended tag block");
    };
    block.payload[1] = 0x09;

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &reparsed.extensions[0] else {
        panic!("expected CTA-861 extension");
    };
    let DataBlock::Extended(block) = &cta.data_blocks[4] else {
        panic!("expected extended tag block");
    };

    assert_eq!(block.payload, vec![0x03, 0x09]);
}

#[test]
fn serializes_hdr_static_metadata_mutation() {
    let mut edid = Edid::parse(&make_hdr_static_metadata_sample()).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let hdr_block = cta.data_blocks.iter_mut().find_map(|block| match block {
        DataBlock::HdrStaticMetadata(block) => Some(block),
        _ => None,
    });
    let hdr_block = hdr_block.expect("expected HDR static metadata block");
    hdr_block.desired_content_max_luminance = Some(0x70);
    hdr_block.desired_content_min_luminance = Some(0x08);

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &reparsed.extensions[0] else {
        panic!("expected CTA-861 extension");
    };
    let hdr_block = cta.data_blocks.iter().find_map(|block| match block {
        DataBlock::HdrStaticMetadata(block) => Some(block),
        _ => None,
    });
    let hdr_block = hdr_block.expect("expected HDR static metadata block");

    assert_eq!(hdr_block.desired_content_max_luminance, Some(0x70));
    assert_eq!(hdr_block.desired_content_min_luminance, Some(0x08));
}

#[test]
fn adds_and_removes_cta_video_vics_via_api() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24_60HZ).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    assert!(cta.add_video_vic(0x22));
    assert!(!cta.add_video_vic(0x22));
    assert!(cta.remove_video_vic(0x07));
    assert!(!cta.remove_video_vic(0x07));

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &reparsed.extensions[0] else {
        panic!("expected CTA-861 extension");
    };
    let DataBlock::Video(video) = &cta.data_blocks[0] else {
        panic!("expected video block");
    };

    assert!(video.vic_codes.contains(&0x22));
    assert!(!video.vic_codes.contains(&0x07));
}

#[test]
fn updates_speaker_allocation_via_api() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    cta.set_speaker_allocation(&[0x05, 0x00, 0x00]);

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &reparsed.extensions[0] else {
        panic!("expected CTA-861 extension");
    };
    let DataBlock::SpeakerAllocation(speakers) = &cta.data_blocks[2] else {
        panic!("expected speaker allocation block");
    };

    assert_eq!(speakers.bytes, vec![0x05, 0x00, 0x00]);
}

#[test]
fn updates_hdmi_max_tmds_clock_via_api() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    cta.set_hdmi_max_tmds_clock_mhz(300).unwrap();

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &reparsed.extensions[0] else {
        panic!("expected CTA-861 extension");
    };
    let DataBlock::Vendor(vendor) = &cta.data_blocks[3] else {
        panic!("expected vendor block");
    };

    assert_eq!(vendor.hdmi.as_ref().unwrap().max_tmds_clock_mhz, Some(300));
}

#[test]
fn updates_hdmi_content_types_via_api() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    cta.set_hdmi_content_types(true, false, true, true).unwrap();

    let bytes = edid.to_bytes();
    let reparsed = Edid::parse(&bytes).unwrap();
    let ExtensionBlock::Cta861(cta) = &reparsed.extensions[0] else {
        panic!("expected CTA-861 extension");
    };
    let DataBlock::Vendor(vendor) = &cta.data_blocks[3] else {
        panic!("expected vendor block");
    };
    let hdmi = vendor.hdmi.as_ref().unwrap();

    assert!(hdmi.hdmi_video_present);
    assert!(hdmi.cnc_graphics);
    assert!(!hdmi.cnc_photo);
    assert!(hdmi.cnc_cinema);
    assert!(hdmi.cnc_game);
}

#[test]
fn rejects_invalid_hdmi_max_tmds_clock() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    let error = cta.set_hdmi_max_tmds_clock_mhz(302).unwrap_err();
    assert!(matches!(error, EdidError::ValidationError(_)));
}

#[test]
fn rejects_monitor_name_longer_than_13_characters() {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let error = edid.set_monitor_name("0123456789ABCD").unwrap_err();
    assert!(matches!(error, EdidError::ValidationError(_)));
}

#[test]
fn rejects_invalid_header() {
    let mut bad = EDID_600M_U2418;
    bad[0] = 0x01;

    let error = Edid::parse(&bad).unwrap_err();
    assert_eq!(error, EdidError::InvalidHeader);
}

#[test]
fn rejects_invalid_checksum() {
    let mut bad = EDID_600M_U2418;
    bad[10] ^= 0xff;

    let error = Edid::parse(&bad).unwrap_err();
    assert!(matches!(
        error,
        EdidError::InvalidChecksum { block_index: 0 }
    ));
}

#[test]
fn rejects_extension_count_mismatch() {
    let mut bad = EDID_600M_U2418;
    bad[126] = 1;
    fix_checksum(&mut bad);

    let error = Edid::parse(&bad).unwrap_err();
    assert_eq!(
        error,
        EdidError::ExtensionCountMismatch {
            expected: 1,
            actual: 0,
        }
    );
}

#[test]
fn rejects_invalid_length() {
    let error = Edid::parse(&EDID_600M_U2418[..64]).unwrap_err();
    assert_eq!(error, EdidError::InvalidLength);
}

#[test]
fn rejects_truncated_cta_data_block() {
    let mut bad = EDID_FORCE_GBR24;
    bad[130] = 0x08;
    let mut extension = [0_u8; 128];
    extension.copy_from_slice(&bad[128..256]);
    fix_checksum(&mut extension);
    bad[128..256].copy_from_slice(&extension);

    let error = Edid::parse(&bad).unwrap_err();
    assert!(matches!(error, EdidError::ParseError(_)));
}

#[test]
fn rejects_truncated_displayid_payload() {
    let mut bytes = make_displayid_sample();
    bytes[128 + 3] = 0x02;
    let mut extension = [0_u8; 128];
    extension.copy_from_slice(&bytes[128..256]);
    fix_checksum(&mut extension);
    bytes[128..256].copy_from_slice(&extension);

    let error = Edid::parse(&bytes).unwrap_err();
    assert!(matches!(error, EdidError::ParseError(_)));
}

fn fix_checksum(block: &mut [u8; 128]) {
    let sum = block[..127]
        .iter()
        .fold(0_u8, |acc, byte| acc.wrapping_add(*byte));
    block[127] = (0_u8).wrapping_sub(sum);
}

fn make_displayid_sample() -> Vec<u8> {
    let mut bytes = EDID_600M_U2418.to_vec();
    bytes[126] = 1;
    let mut base = [0_u8; 128];
    base.copy_from_slice(&bytes[..128]);
    fix_checksum(&mut base);
    bytes[..128].copy_from_slice(&base);

    let mut extension = [0_u8; 128];
    extension[0] = 0x70;
    extension[1] = 0x01;
    extension[2] = 0x03;
    extension[3] = 0x0b;
    extension[4] = 0x02;
    extension[5] = 0x00;
    extension[6] = 0x20;
    extension[7] = 0x01;
    extension[8] = 0x03;
    extension[9] = 0x12;
    extension[10] = 0x34;
    extension[11] = 0x56;
    extension[12] = 0x7f;
    extension[13] = 0x00;
    extension[14] = 0x02;
    extension[15] = 0xaa;
    extension[16] = 0xbb;
    fix_checksum(&mut extension);

    bytes.extend_from_slice(&extension);
    bytes
}

fn make_hdr_static_metadata_sample() -> Vec<u8> {
    let mut edid = Edid::parse(&EDID_FORCE_GBR24).unwrap();
    let ExtensionBlock::Cta861(cta) = &mut edid.extensions[0] else {
        panic!("expected CTA-861 extension");
    };

    cta.data_blocks.push(DataBlock::HdrStaticMetadata(
        edidkit::HdrStaticMetadataBlock {
            electro_optical_transfer_functions: 0x05,
            static_metadata_descriptors: 0x01,
            desired_content_max_luminance: Some(0x64),
            desired_content_max_frame_average_luminance: Some(0x32),
            desired_content_min_luminance: Some(0x0a),
        },
    ));

    edid.to_bytes()
}
