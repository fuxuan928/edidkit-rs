use std::{env, fs, path::Path};

#[path = "support/demo_edid.rs"]
mod demo_edid;

use edidkit::base::{Descriptor, VideoInputDefinition};
use edidkit::displayid::DisplayIdDataBlockKind;
use edidkit::{Edid, ExtensionBlock};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (input_source, output_path, monitor_name) = parse_args()?;

    let input_bytes = match &input_source {
        InputSource::File(path) => fs::read(path)?,
        InputSource::BuiltIn => demo_edid::DEMO_EDID.to_vec(),
    };
    let mut edid = Edid::parse(&input_bytes)?;

    println!(
        "Loaded {} bytes from {}",
        input_bytes.len(),
        input_source.display()
    );
    print_summary("Original", &edid);

    let mut product = edid.product();
    product.monitor_name = Some(monitor_name);
    edid.set_product(&product)?;
    let output_bytes = edid.to_bytes()?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, &output_bytes)?;

    println!(
        "Saved {} bytes to {}",
        output_bytes.len(),
        output_path.display()
    );

    let updated = Edid::parse(&output_bytes)?;
    print_summary("Updated", &updated);

    Ok(())
}

fn parse_args() -> Result<(InputSource, std::path::PathBuf, String), Box<dyn std::error::Error>> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let input_path = match args.next() {
        Some(value) => InputSource::File(value.into()),
        None => {
            println!(
                "No input provided, using built-in demo EDID and writing to {} with name {}",
                demo_edid::default_output_path().display(),
                demo_edid::DEMO_MONITOR_NAME
            );
            return Ok((
                InputSource::BuiltIn,
                demo_edid::default_output_path(),
                demo_edid::DEMO_MONITOR_NAME.to_owned(),
            ));
        }
    };
    let Some(output_path) = args.next() else {
        print_usage(&program);
        return Err("missing output EDID path".into());
    };
    let Some(monitor_name) = args.next() else {
        print_usage(&program);
        return Err("missing monitor name".into());
    };

    if args.next().is_some() {
        print_usage(&program);
        return Err("too many arguments".into());
    }

    Ok((
        input_path,
        output_path.into(),
        monitor_name.to_string_lossy().into_owned(),
    ))
}

fn print_usage(program: &std::ffi::OsStr) {
    eprintln!(
        "Usage: {} [input.edid output.edid new-monitor-name]",
        Path::new(program)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("edit_monitor_name")
    );
    eprintln!("If no arguments are given, a built-in demo EDID is used.");
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

fn print_summary(label: &str, edid: &Edid) {
    println!("\n{label} EDID Summary");
    println!("manufacturer: {}", edid.base.manufacturer_id.0);
    println!("product code: 0x{:04x}", edid.base.product_code);
    println!("serial number: 0x{:08x}", edid.base.serial_number);
    println!(
        "manufacture date: week {} year {}",
        edid.base.manufacture_date.week, edid.base.manufacture_date.year
    );
    println!("video input: {}", video_input_summary(edid));
    println!(
        "monitor name: {}",
        edid.monitor_name().unwrap_or("<not present>")
    );
    println!(
        "monitor serial: {}",
        edid.monitor_serial().unwrap_or("<not present>")
    );
    println!("extensions: {}", edid.extensions.len());
    print_descriptors(edid);

    for (index, extension) in edid.extensions.iter().enumerate() {
        match extension {
            ExtensionBlock::Cta861(cta) => {
                println!(
                    "  [{index}] CTA-861 revision {}, data blocks {}",
                    cta.revision,
                    cta.data_blocks.len()
                );
            }
            ExtensionBlock::DisplayId(display_id) => {
                println!(
                    "  [{index}] DisplayID v{}.{} blocks {}",
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
                        "      - tag 0x{:02x}, revision {}, payload {}, kind {}",
                        block.tag,
                        block.revision,
                        block.payload.len(),
                        kind
                    );
                }
            }
            ExtensionBlock::Unknown(bytes) => {
                println!("  [{index}] Unknown extension ({} bytes)", bytes.len());
            }
        }
    }
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
            Descriptor::Unused => println!("  [{index}] unused descriptor slot"),
            Descriptor::Unknown(_) => println!("  [{index}] unknown descriptor"),
        }
    }
}
