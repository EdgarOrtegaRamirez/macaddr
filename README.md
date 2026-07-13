# macaddr

**MAC address toolkit** — generate, parse, convert, validate, and look up MAC addresses from the command line.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Features

- **Parse** — Parse MAC addresses in any format (colon, hyphen, dot, raw)
- **Generate** — Create random or vendor-specific MAC addresses
- **Convert** — Convert between formats (colon, hyphen, dot, cisco, raw)
- **Validate** — Validate MAC address strings (exit code 0/1 for scripting)
- **Vendor Lookup** — Identify the OUI vendor (100+ built-in vendors: Cisco, Apple, VMware, Intel, etc.)
- **Arithmetic** — Increment and decrement MAC addresses
- **EUI-64** — Generate EUI-64 interface identifiers from MAC addresses
- **Details** — Show unicast/multicast, local/universal, broadcast/null info
- **Multiple Generation** — Generate multiple addresses at once

## Installation

```bash
cargo install macaddr
```

Or build from source:

```bash
git clone https://github.com/EdgarOrtegaRamirez/macaddr.git
cd macaddr
cargo build --release
./target/release/macaddr --help
```

## Usage

### Parse a MAC address

```bash
macaddr parse 00:11:22:33:44:55
```

Output:
```
MAC Address: 00:11:22:33:44:55
  OUI:             00:11:22 (Unknown)
  NIC:             33:44:55
  Type:            Unicast
  Admin:           Universally administered
  Broadcast:       false
  Null:            false

Formats:
  Colon:  00:11:22:33:44:55
  Hyphen: 00-11-22-33-44-55
  Dot:    0011.2233.4455
  Cisco:  0011.2233.4455
  Raw:    001122334455
```

### Generate random MAC addresses

```bash
# Generate one random MAC
macaddr generate

# Generate 5 random MACs
macaddr generate --count 5

# Generate as Cisco format
macaddr generate --format cisco

# Generate for a specific vendor
macaddr generate --vendor cisco
macaddr generate --vendor apple
macaddr generate --vendor vmware
```

### Convert format

```bash
# Convert to hyphen format
macaddr convert 00:11:22:33:44:55 --to hyphen

# Show all formats
macaddr convert 00:11:22:33:44:55 --all
```

### Validate

```bash
macaddr validate 00:11:22:33:44:55 && echo "valid"
echo $?  # 0 = valid, 1 = invalid
```

### Vendor lookup

```bash
# By MAC address
macaddr vendor 00:0C:29:12:34:56

# By OUI prefix
macaddr vendor 00:0C:29

# List all known vendors
macaddr list-vendors

# Search vendors
macaddr list-vendors --search cisco
```

### Increment / Decrement

```bash
macaddr next 00:00:00:00:00:FF
macaddr prev 00:00:00:00:00:01
```

### EUI-64 conversion

```bash
macaddr eui64 00:11:22:33:44:55
# Output: 02:11:22:ff:fe:33:44:55
```

## Supported Formats

| Format | Example | Description |
|--------|---------|-------------|
| Colon | `00:11:22:33:44:55` | Standard IEEE 802 format |
| Hyphen | `00-11-22-33-44-55` | Windows/HP format |
| Dot | `0011.2233.4455` | Cisco format |
| Cisco | `0011.2233.4455` | Same as dot |
| Raw | `001122334455` | No separators |

## Built-in Vendors (100+)

Includes OUIs for Cisco, Apple, VMware, Intel, Dell, HP, Google, Microsoft, AWS/Amazon, Samsung, Huawei, Netgear, Ubiquiti, Raspberry Pi, Aruba, Palo Alto Networks, Fortinet, Oracle, IBM, and many more.

## License

MIT