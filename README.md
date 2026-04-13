# edidkit

`edidkit` is a Rust library for parsing, inspecting, editing, and re-serializing EDID data.

The project targets practical EDID workflows: keep unknown data intact, expose known fields through typed APIs, and make safe round-trip serialization possible.

## Why edidkit

EDID support is rarely all-or-nothing. Real-world data mixes base EDID, extension blocks, vendor-specific content, and partially documented fields.

`edidkit` is built around a simple rule:

> parse what is understood, preserve what is not.

## Current Features

- Parse EDID base blocks
- Parse common base block descriptors
- Parse CTA-861 extension blocks
- Edit selected base block fields
- Edit selected CTA-861 fields
- Serialize EDID back to bytes
- Validate checksums for all 128-byte blocks
- Preserve raw bytes for round-trip safety

## Supported Today

Base EDID support includes:

- manufacturer ID
- product code
- serial number
- manufacture date
- EDID version
- descriptor parsing for:
  - detailed timings
  - monitor name
  - monitor serial
  - range limits
  - unknown descriptors

CTA-861 support includes:

- extension header parsing
- video data blocks
- audio data blocks
- speaker allocation blocks
- extended tag blocks
- HDR static metadata blocks
- vendor-specific data blocks as `OUI + payload`
- HDMI VSDB decoding for physical address, deep color, content types, and latency fields
- unknown data blocks preserved as raw payload

DisplayID support includes:

- extension header parsing
- typed DisplayID data block envelopes
- product data block detection
- vendor-specific data block detection
- raw extension preservation for conservative write-back

Editing support currently includes:

- `set_product_code`
- `set_monitor_name`
- `Cta861Extension::add_video_vic`
- `Cta861Extension::remove_video_vic`
- `Cta861Extension::set_speaker_allocation`
- `Cta861Extension::set_hdmi_max_tmds_clock_mhz`
- `Cta861Extension::set_hdmi_content_types`

## Installation

This crate is not published yet.

Use it from a local path or git dependency for now.

Example with a local path:

```toml
[dependencies]
edidkit = { path = "../edidkit" }
```

## Quick Start

Parse EDID bytes:

```rust
use edidkit::Edid;

fn inspect(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let edid = Edid::parse(bytes)?;

    println!("manufacturer: {}", edid.base.manufacturer_id.0);
    println!("product code: {}", edid.base.product_code);
    println!("extensions: {}", edid.extensions.len());

    Ok(())
}
```

Edit and re-serialize:

```rust
use edidkit::Edid;

fn rewrite(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut edid = Edid::parse(bytes)?;

    edid.set_product_code(0x4321);
    edid.set_monitor_name("RK-UHD-ALT")?;

    Ok(edid.to_bytes())
}
```

Edit CTA-861 data blocks:

```rust
use edidkit::{Edid, ExtensionBlock};

fn patch_cta(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut edid = Edid::parse(bytes)?;

    for extension in &mut edid.extensions {
        if let ExtensionBlock::Cta861(cta) = extension {
            cta.add_video_vic(0x22);
            cta.remove_video_vic(0x07);
            cta.set_speaker_allocation(&[0x05, 0x00, 0x00]);
            cta.set_hdmi_max_tmds_clock_mhz(300)?;
        }
    }

    Ok(edid.to_bytes())
}
```

Inspect extension types:

```rust
use edidkit::{DataBlock, Edid, ExtensionBlock};

fn print_extensions(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let edid = Edid::parse(bytes)?;

    for extension in &edid.extensions {
        if let ExtensionBlock::Cta861(cta) = extension {
            for block in &cta.data_blocks {
                match block {
                    DataBlock::Video(video) => println!("video VICs: {:?}", video.vic_codes),
                    DataBlock::Audio(audio) => {
                        println!("audio descriptors: {}", audio.short_audio_descriptors.len())
                    }
                    DataBlock::Vendor(vendor) => {
                        println!("vendor OUI: {:02x?}", vendor.oui);
                        if let Some(hdmi) = &vendor.hdmi {
                            println!("max TMDS clock: {:?} MHz", hdmi.max_tmds_clock_mhz);
                        }
                    }
                    DataBlock::SpeakerAllocation(speakers) => {
                        println!("speaker allocation: {:02x?}", speakers.bytes)
                    }
                    DataBlock::HdrStaticMetadata(block) => {
                        println!(
                            "HDR EOTF flags: {:02x}, descriptors: {:02x}",
                            block.electro_optical_transfer_functions,
                            block.static_metadata_descriptors
                        )
                    }
                    DataBlock::Extended(block) => {
                        println!("extended CTA tag: {}", block.extended_tag)
                    }
                    DataBlock::Unknown { tag, .. } => println!("unknown CTA block tag: {}", tag),
                }
            }
        }
    }

    Ok(())
}
```

## Design

The core model keeps both typed data and raw bytes:

```rust
pub struct Edid {
    pub raw: Vec<u8>,
    pub base: BaseBlock,
    pub extensions: Vec<ExtensionBlock>,
}
```

Known extensions are typed, unknown extensions are preserved:

```rust
pub enum ExtensionBlock {
    Cta861(Cta861Extension),
    DisplayId(DisplayIdExtension),
    Unknown(Vec<u8>),
}
```

This allows the crate to expose structured APIs without forcing full coverage of every EDID variant from day one.

## Round-Trip Behavior

For unmodified input, the crate aims to preserve exact bytes:

```text
parse -> to_bytes == original bytes
```

Current write behavior is intentionally conservative:

- the base block is rewritten from the typed model
- CTA-861 data blocks are rewritten from the typed model when they can be encoded safely
- DisplayID is currently parsed into typed data block envelopes and written back from preserved raw bytes
- checksums are recomputed automatically
- unsupported extension types continue to use preserved raw bytes

That keeps writes predictable while allowing real CTA edits on supported blocks.

## Validation

The parser validates:

- EDID length must be a multiple of 128 bytes
- EDID header must match the standard header
- every 128-byte block checksum must be valid
- extension count must match the actual number of extension blocks
- monitor name text must be ASCII and no longer than 13 characters

## Limitations

Current limitations are intentional:

- HDMI vendor-specific payloads are only partially decoded today
- DisplayID data blocks are only partially typed today
- there is no CLI yet
- there is no hardware integration layer

## Roadmap

Planned next steps:

1. Decode more HDMI VSDB and extended CTA fields
2. Add richer typed DisplayID block support
3. Add broader fuzz testing and sample coverage
4. Add optional CLI tooling for inspection and patching
5. Publish the crate and stabilize the public editing API

## Development

Run tests with:

```bash
cargo test
```

Run fuzzing with `cargo-fuzz`:

```bash
cargo install cargo-fuzz
cargo fuzz run parse_edid
```

Project layout:

```text
src/
├── error.rs
├── lib.rs
├── model/
├── parser/
├── utils/
└── writer/
```

Fuzz targets live under `fuzz/`.

## License

No license file has been added yet.
