use clap::{Parser, Subcommand};
use macaddr::models::{MacAddr, MacAddrFormat};
use macaddr::vendor;

/// MAC address toolkit — generate, parse, convert, validate, and look up MAC addresses
#[derive(Parser)]
#[command(name = "macaddr", about = "MAC address toolkit", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and display details about a MAC address
    Parse {
        /// MAC address to parse
        mac: String,
    },
    /// Generate a random MAC address
    Generate {
        /// Generate multiple MAC addresses
        #[arg(short, long, default_value = "1")]
        count: usize,

        /// Generate for a specific vendor name (e.g., "cisco", "apple", "vmware")
        #[arg(short, long)]
        vendor: Option<String>,

        /// Output format
        #[arg(short, long, default_value = "colon")]
        format: Option<MacAddrFormat>,
    },
    /// Convert a MAC address to a different format
    Convert {
        /// MAC address to convert
        mac: String,

        /// Output format (colon, hyphen, dot, cisco, raw)
        #[arg(short, long, default_value = "colon")]
        to: MacAddrFormat,

        /// Also show all formats
        #[arg(short, long)]
        all: bool,
    },
    /// Look up vendor/OUI information for a MAC address
    Vendor {
        /// MAC address or OUI to look up
        mac: String,
    },
    /// List all known OUI vendors
    ListVendors {
        /// Search filter
        #[arg(short, long)]
        search: Option<String>,
    },
    /// Validate a MAC address string
    Validate {
        /// MAC address to validate
        mac: String,
    },
    /// Increment a MAC address by 1
    Next {
        /// MAC address
        mac: String,
    },
    /// Decrement a MAC address by 1
    Prev {
        /// MAC address
        mac: String,
    },
    /// Generate EUI-64 interface identifier from a MAC
    Eui64 {
        /// MAC address to convert
        mac: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { mac } => cmd_parse(&mac),
        Commands::Generate { count, vendor, format } => cmd_generate(count, vendor, format),
        Commands::Convert { mac, to, all } => cmd_convert(&mac, to, all),
        Commands::Vendor { mac } => cmd_vendor(&mac),
        Commands::ListVendors { search } => cmd_list_vendors(search),
        Commands::Validate { mac } => cmd_validate(&mac),
        Commands::Next { mac } => cmd_next(&mac),
        Commands::Prev { mac } => cmd_prev(&mac),
        Commands::Eui64 { mac } => cmd_eui64(&mac),
    }
}

fn parse_mac(s: &str) -> Result<MacAddr, String> {
    s.parse::<MacAddr>().map_err(|e| e.to_string())
}

fn cmd_parse(s: &str) {
    match parse_mac(s) {
        Ok(mac) => {
            println!("MAC Address: {}", mac);
            println!("  OUI:             {} ({})", format_oui(&mac), vendor_str(&mac));
            println!("  NIC:             {}", mac.format_hyphen().replace('-', ":").split(':').skip(3).collect::<Vec<_>>().join(":"));
            println!("  Type:            {}", mac_type(&mac));
            println!("  Admin:           {}", admin_type(&mac));
            println!("  Broadcast:       {}", mac.is_broadcast());
            println!("  Null:            {}", mac.is_null());
            println!();
            println!("Formats:");
            println!("  Colon:  {}", mac.format_colon());
            println!("  Hyphen: {}", mac.format_hyphen());
            println!("  Dot:    {}", mac.format_dot());
            println!("  Cisco:  {}", mac.format_cisco());
            println!("  Raw:    {}", mac.format_raw());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_generate(count: usize, vendor_name: Option<String>, format: Option<MacAddrFormat>) {
    let fmt = format.unwrap_or(MacAddrFormat::Colon);

    for i in 0..count {
        let mac = if let Some(ref name) = vendor_name {
            match vendor::generate_for_vendor_name(name) {
                Some(m) => m,
                None => {
                    eprintln!("Vendor '{}' not found. Use 'macaddr list-vendors' to see available vendors.", name);
                    return;
                }
            }
        } else {
            vendor::generate_random()
        };

        let output = match fmt {
            MacAddrFormat::Colon => mac.format_colon(),
            MacAddrFormat::Hyphen => mac.format_hyphen(),
            MacAddrFormat::Dot => mac.format_dot(),
            MacAddrFormat::Cisco => mac.format_cisco(),
            MacAddrFormat::Raw => mac.format_raw(),
        };

        if count == 1 {
            println!("{}", output);
        } else {
            println!("{}  {}", i + 1, output);
        }
    }
}

fn cmd_convert(s: &str, to: MacAddrFormat, all: bool) {
    match parse_mac(s) {
        Ok(mac) => {
            if all {
                println!("Colon:  {}", mac.format_colon());
                println!("Hyphen: {}", mac.format_hyphen());
                println!("Dot:    {}", mac.format_dot());
                println!("Cisco:  {}", mac.format_cisco());
                println!("Raw:    {}", mac.format_raw());
            } else {
                let output = match to {
                    MacAddrFormat::Colon => mac.format_colon(),
                    MacAddrFormat::Hyphen => mac.format_hyphen(),
                    MacAddrFormat::Dot => mac.format_dot(),
                    MacAddrFormat::Cisco => mac.format_cisco(),
                    MacAddrFormat::Raw => mac.format_raw(),
                };
                println!("{}", output);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_vendor(s: &str) {
    match parse_mac(s) {
        Ok(mac) => {
            let oui = mac.oui();
            let hex = format!("{:02X}{:02X}{:02X}", oui[0], oui[1], oui[2]);
            match vendor::lookup_vendor(&oui) {
                Some(name) => println!("OUI {}: {}", hex, name),
                None => println!("OUI {}: Unknown vendor", hex),
            }
        }
        Err(_) => {
            // Maybe it's just an OUI
            let cleaned: String = s.chars().filter(|&c| c != ':' && c != '-' && c != '.').collect();
            if cleaned.len() >= 6 {
                let oui_hex = &cleaned[..6].to_uppercase();
                if let Ok(_) = u64::from_str_radix(oui_hex, 16) {
                    let vendor_list = macaddr::vendor::list_vendors();
                    let found = vendor_list.iter().find(|&&(k, _)| k == oui_hex);
                    match found {
                        Some(&(_, name)) => println!("OUI {}: {}", oui_hex, name),
                        None => println!("OUI {}: Unknown", oui_hex),
                    }
                } else {
                    eprintln!("Error: invalid OUI '{}'", s);
                }
            } else {
                eprintln!("Error: could not parse '{}' as a MAC address or OUI", s);
            }
        }
    }
}

fn cmd_list_vendors(search: Option<String>) {
    let vendors = vendor::list_vendors();
    match search {
        None => {
            println!("{:<10} {}", "OUI", "Vendor");
            println!("{}", "-".repeat(60));
            for (oui, name) in &vendors {
                println!("{:<10} {}", oui, name);
            }
            println!("\nTotal: {} vendors", vendors.len());
        }
        Some(query) => {
            let q = query.to_lowercase();
            let filtered: Vec<_> = vendors.iter().filter(|&&(_, n)| n.to_lowercase().contains(&q)).collect();
            if filtered.is_empty() {
                println!("No vendors matching '{}'", query);
            } else {
                println!("{:<10} {}", "OUI", "Vendor");
                println!("{}", "-".repeat(60));
                for (oui, name) in &filtered {
                    println!("{:<10} {}", oui, name);
                }
                println!("\nFound: {} vendors", filtered.len());
            }
        }
    }
}

fn cmd_validate(s: &str) {
    match s.parse::<MacAddr>() {
        Ok(mac) => {
            println!("✓ Valid MAC address: {}", mac);
            println!("  Type:  {}", mac_type(&mac));
            println!("  Admin: {}", admin_type(&mac));
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("✗ Invalid MAC address: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_next(s: &str) {
    match parse_mac(s) {
        Ok(mac) => println!("{}", mac.increment()),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_prev(s: &str) {
    match parse_mac(s) {
        Ok(mac) => println!("{}", mac.decrement()),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_eui64(s: &str) {
    match parse_mac(s) {
        Ok(mac) => {
            let eui64 = mac.to_eui64();
            println!("MAC:    {}", mac);
            println!("EUI-64: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                eui64[0], eui64[1], eui64[2], eui64[3], eui64[4], eui64[5], eui64[6], eui64[7]);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn format_oui(mac: &MacAddr) -> String {
    let oui = mac.oui();
    format!("{:02X}:{:02X}:{:02X}", oui[0], oui[1], oui[2])
}

fn vendor_str(mac: &MacAddr) -> &str {
    vendor::lookup_vendor(&mac.oui()).unwrap_or("Unknown")
}

fn mac_type(mac: &MacAddr) -> &str {
    if mac.is_broadcast() {
        "Broadcast"
    } else if mac.is_multicast() {
        "Multicast"
    } else {
        "Unicast"
    }
}

fn admin_type(mac: &MacAddr) -> &str {
    if mac.is_local() {
        "Locally administered"
    } else {
        "Universally administered"
    }
}