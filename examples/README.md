# Examples

This directory contains runnable example programs for `edidkit`.

## Available Examples

### `print_extensions`

Inspect one EDID and print:

- manufacturer ID
- product code
- hardware serial number (`serial_number`)
- manufacture week/year
- video input summary
- monitor name descriptor
- monitor serial descriptor
- descriptor overview
- extension count
- CTA-861 blocks
- DisplayID blocks

Run with the built-in demo EDID:

```bash
cargo run --example print_extensions
```

Run with your own EDID file:

```bash
cargo run --example print_extensions -- input.edid
```

`input.edid` means a path to a raw EDID binary file.

### `edit_monitor_name`

Read one EDID, print a summary, update the monitor name, save the new file, then print the updated summary.

The summary includes:

- manufacturer ID
- product code
- hardware serial number (`serial_number`)
- manufacture week/year
- video input summary
- monitor name descriptor
- monitor serial descriptor
- descriptor overview
- extension summary

Run with the built-in demo EDID:

```bash
cargo run --example edit_monitor_name
```

Default built-in demo behavior:

- input: built-in demo EDID
- output: `target/examples/edited-monitor-name.edid`
- new monitor name: `RK-UHD-ALT`

Run with your own file paths and monitor name:

```bash
cargo run --example edit_monitor_name -- input.edid output.edid RK-UHD-ALT
```

Arguments:

- `input.edid`: source EDID binary file
- `output.edid`: destination EDID binary file
- `RK-UHD-ALT`: new monitor name string

## Notes

- The examples are intended for demonstration and inspection workflows.
- The built-in demo EDID is valid and can be used without preparing sample files first.
