use std::{env, fs, path::Path};

#[path = "support/demo_edid.rs"]
mod demo_edid;

use edidkit::base::{Descriptor, VideoInputDefinition};
use edidkit::cta861::DataBlock;
use edidkit::displayid::DisplayIdDataBlockKind;
use edidkit::{Edid, ExtensionBlock};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_source = parse_args()?;
    let bytes = match &input_source {
        InputSource::File(path) => fs::read(path)?,
        InputSource::BuiltIn => demo_edid::DEMO_EDID.to_vec(),
    };
    let edid = Edid::parse(&bytes)?;

    println!(
        "Loaded {} bytes from {}",
        bytes.len(),
        input_source.display()
    );
    print_base_summary(&edid);
    print_extensions(&edid);

    Ok(())
}

fn parse_args() -> Result<InputSource, Box<dyn std::error::Error>> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let Some(input_path) = args.next() else {
        println!("No input provided, using built-in demo EDID.");
        return Ok(InputSource::BuiltIn);
    };

    if args.next().is_some() {
        print_usage(&program);
        return Err("too many arguments".into());
    }

    Ok(InputSource::File(input_path.into()))
}

fn print_usage(program: &std::ffi::OsStr) {
    eprintln!(
        "Usage: {} [input.edid]",
        Path::new(program)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("print_extensions")
    );
    eprintln!("If no input is given, a built-in demo EDID is used.");
}

enum InputSource {
    File(std::path::PathBuf),
    BuiltIn,
}

impl InputSource {
    fn display(&self) -> String {
        match self {
            Self::File(path) => path.display().to_string(),
            Self::BuiltIn => "built-in demo EDID".to_owned(),
        }
    }
}

fn print_base_summary(edid: &Edid) {
    println!("\nBase EDID");
    println!("manufacturer: {}", edid.base.manufacturer_id.0);
    println!("product code: 0x{:04x}", edid.base.product_code);
    println!("serial number: 0x{:08x}", edid.base.serial_number);
    println!(
        "manufacture date: week {} year {}",
        edid.base.manufacture_date.week, edid.base.manufacture_date.year
    );
    println!(
        "version: {}.{}",
        edid.base.version.major, edid.base.version.minor
    );
    println!("video input: {}", video_input_summary(edid));
    println!(
        "monitor name: {}",
        monitor_name(edid).unwrap_or("<not present>")
    );
    println!(
        "monitor serial: {}",
        monitor_serial(edid).unwrap_or("<not present>")
    );
    println!("extensions: {}", edid.extensions.len());
    print_descriptors(edid);
}

fn print_extensions(edid: &Edid) {
    println!("\nExtensions");

    for (index, extension) in edid.extensions.iter().enumerate() {
        match extension {
            ExtensionBlock::Cta861(cta) => print_cta_extension(index, cta),
            ExtensionBlock::DisplayId(display_id) => print_displayid_extension(index, display_id),
            ExtensionBlock::Unknown(bytes) => {
                println!("[{index}] Unknown extension ({} bytes)", bytes.len());
            }
        }
    }
}

fn print_cta_extension(index: usize, cta: &edidkit::cta861::Cta861Extension) {
    println!(
        "[{index}] CTA-861 revision {}, data blocks {}",
        cta.revision,
        cta.data_blocks.len()
    );

    for block in &cta.data_blocks {
        match block {
            DataBlock::Video(video) => println!("  - video VICs: {:?}", video.vic_codes),
            DataBlock::Audio(audio) => println!(
                "  - audio descriptors: {}",
                audio.short_audio_descriptors.len()
            ),
            DataBlock::Vendor(vendor) => {
                println!("  - vendor OUI: {:02x?}", vendor.oui);
                if let Some(hdmi) = &vendor.hdmi {
                    println!(
                        "    HDMI VSDB: max TMDS {:?} MHz, video_present {}",
                        hdmi.max_tmds_clock_mhz, hdmi.hdmi_video_present
                    );
                }
            }
            DataBlock::SpeakerAllocation(speakers) => {
                println!("  - speaker allocation: {:02x?}", speakers.bytes)
            }
            DataBlock::HdrStaticMetadata(block) => println!(
                "  - HDR static metadata: eotf=0x{:02x} descriptors=0x{:02x}",
                block.electro_optical_transfer_functions, block.static_metadata_descriptors
            ),
            DataBlock::Extended(block) => {
                println!(
                    "  - extended CTA tag {} payload {:02x?}",
                    block.extended_tag, block.payload
                )
            }
            DataBlock::Unknown { tag, payload } => {
                println!("  - unknown CTA block tag {} payload {:02x?}", tag, payload)
            }
        }
    }
}

fn print_displayid_extension(index: usize, display_id: &edidkit::displayid::DisplayIdExtension) {
    println!(
        "[{index}] DisplayID v{}.{} blocks {}",
        display_id.version,
        display_id.revision,
        display_id.data_blocks.len()
    );

    for block in &display_id.data_blocks {
        let kind = match &block.kind {
            DisplayIdDataBlockKind::Product(_) => "product",
            DisplayIdDataBlockKind::VendorSpecific(_) => "vendor-specific",
            DisplayIdDataBlockKind::Unknown => "unknown",
        };
        println!(
            "  - tag 0x{:02x}, revision {}, payload {}, kind {}",
            block.tag,
            block.revision,
            block.payload.len(),
            kind
        );
    }
}

fn monitor_name(edid: &Edid) -> Option<&str> {
    edid.base
        .descriptors
        .iter()
        .find_map(|descriptor| match descriptor {
            Descriptor::MonitorName(name) => Some(name.as_str()),
            _ => None,
        })
}

fn monitor_serial(edid: &Edid) -> Option<&str> {
    edid.base
        .descriptors
        .iter()
        .find_map(|descriptor| match descriptor {
            Descriptor::MonitorSerial(serial) => Some(serial.as_str()),
            _ => None,
        })
}

fn video_input_summary(edid: &Edid) -> String {
    match &edid.base.video_input_definition {
        VideoInputDefinition::Analog(input) => format!(
            "analog (separate_sync={}, composite_hsync={}, composite_green={}, serration={})",
            input.separate_sync_supported,
            input.composite_sync_on_hsync_supported,
            input.composite_sync_on_green_supported,
            input.serration_supported
        ),
        VideoInputDefinition::Digital(input) => format!(
            "digital (dfp_1x_compatible={}, color_bit_depth={:?}, interface={:?})",
            input.dfp_1x_compatible, input.color_bit_depth, input.interface
        ),
    }
}

fn print_descriptors(edid: &Edid) {
    println!("descriptors:");

    for (index, descriptor) in edid.base.descriptors.iter().enumerate() {
        match descriptor {
            Descriptor::DetailedTiming(timing) => println!(
                "  [{index}] detailed timing: {} kHz, {}x{} active",
                timing.pixel_clock_khz, timing.horizontal_active, timing.vertical_active
            ),
            Descriptor::MonitorName(name) => println!("  [{index}] monitor name: {name}"),
            Descriptor::MonitorSerial(serial) => {
                println!("  [{index}] monitor serial: {serial}")
            }
            Descriptor::RangeLimits(range) => println!(
                "  [{index}] range limits: v {}-{} Hz, h {}-{} kHz, max {} MHz",
                range.min_vertical_hz,
                range.max_vertical_hz,
                range.min_horizontal_khz,
                range.max_horizontal_khz,
                range.max_pixel_clock_mhz
            ),
            Descriptor::Unknown(_) => println!("  [{index}] unknown descriptor"),
        }
    }
}
