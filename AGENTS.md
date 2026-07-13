# AGENTS.md — macaddr

## Project Overview

**macaddr** is a Rust CLI tool for MAC address generation, validation, vendor lookup, and format conversion. It handles all common MAC address formats (colon, hyphen, dot, raw) and includes a built-in database of 100+ OUI vendors.

## Quick Commands

```bash
# Parse and show details
macaddr parse "00:11:22:33:44:55"

# Generate random MAC
macaddr generate

# Generate for vendor
macaddr generate --vendor cisco

# Validate (exit code 0/1)
macaddr validate "00:11:22:33:44:55"

# Vendor lookup
macaddr vendor "00:0C:29:12:34:56"

# Convert format
macaddr convert "00:11:22:33:44:55" --to hyphen

# List vendors
macaddr list-vendors --search apple

# Next/prev MAC
macaddr next "FF:FF:FF:FF:FF:FF"

# EUI-64
macaddr eui64 "00:11:22:33:44:55"
```

## Architecture

- `src/main.rs` — CLI entry point using clap derive (9 subcommands)
- `src/models.rs` — MacAddr struct, MacAddrFormat enum, parsing, formatting, arithmetic
- `src/vendor.rs` — OUI vendor database (100+ entries), vendor lookup, MAC generation
- `src/lib.rs` — Library root re-exporting models and vendor
- `tests/test_macaddr.rs` — 39 integration tests

## Key Types

### MacAddr
- Wraps `[u8; 6]` — immutable, Copy
- Parsed from string via `FromStr` (colon, hyphen, dot, raw, mixed)
- Methods: `oui()`, `nic()`, `is_unicast()`, `is_multicast()`, `is_broadcast()`, `is_null()`, `is_local()`, `is_universal()`
- Methods: `increment()`, `decrement()`, `to_eui64()`
- Formatting: `format_colon()`, `format_hyphen()`, `format_dot()`, `format_cisco()`, `format_raw()`

### MacAddrFormat
- Enum: Colon, Hyphen, Dot, Cisco, Raw
- Parsed from `"colon"`, `"hyphen"`, `"dot"`, `"cisco"`, `"raw"`

## Development

```bash
# Build
cargo build

# Test
cargo test

# Run
cargo run -- parse "00:11:22:33:44:55"

# Release build
cargo build --release
```

## Built-in OUI Database

The `vendor.rs` file contains 100+ OUI entries for major vendors. To add a new vendor:
1. Add a `(OUI_HEX, "Vendor Name")` tuple to `BUILTIN_OUIS`
2. The OUI should be 6 uppercase hex characters (e.g., `"000C29"`)

## CLI Commands

| Command | Description |
|---------|-------------|
| parse | Parse and display MAC address details |
| generate | Generate random or vendor-specific MACs |
| convert | Convert between formats |
| vendor | Look up OUI vendor |
| list-vendors | List/search known vendors |
| validate | Validate MAC address (exit code) |
| next | Increment MAC by 1 |
| prev | Decrement MAC by 1 |
| eui64 | Generate EUI-64 from MAC |