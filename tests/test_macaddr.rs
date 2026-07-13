use macaddr::models::MacAddr;
use macaddr::models::MacAddrFormat;
use std::str::FromStr;

// ---- Parsing ----

#[test]
fn test_parse_colon() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(mac.bytes(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
}

#[test]
fn test_parse_hyphen() {
    let mac: MacAddr = "00-11-22-33-44-55".parse().unwrap();
    assert_eq!(mac.bytes(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
}

#[test]
fn test_parse_dot() {
    let mac: MacAddr = "0011.2233.4455".parse().unwrap();
    assert_eq!(mac.bytes(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
}

#[test]
fn test_parse_raw() {
    let mac: MacAddr = "001122334455".parse().unwrap();
    assert_eq!(mac.bytes(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
}

#[test]
fn test_parse_upper() {
    let mac: MacAddr = "AA:BB:CC:DD:EE:FF".parse().unwrap();
    assert_eq!(mac.bytes(), &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
}

#[test]
fn test_parse_lower() {
    let mac: MacAddr = "aa:bb:cc:dd:ee:ff".parse().unwrap();
    assert_eq!(mac.bytes(), &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
}

#[test]
fn test_parse_mixed_separators() {
    let mac: MacAddr = "00-11:22.33-44:55".parse().unwrap();
    assert_eq!(mac.bytes(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
}

#[test]
fn test_parse_errors() {
    assert!("not-a-mac".parse::<MacAddr>().is_err());
    assert!("".parse::<MacAddr>().is_err());
    assert!("00:11:22:33:44".parse::<MacAddr>().is_err());
    assert!("00:11:22:33:44:55:66".parse::<MacAddr>().is_err());
    assert!("00:11:22:33:44:GG".parse::<MacAddr>().is_err());
}

// ---- Property checks ----

#[test]
fn test_is_unicast() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert!(mac.is_unicast());
    assert!(!mac.is_multicast());
}

#[test]
fn test_is_multicast() {
    let mac: MacAddr = "01:00:5E:00:00:01".parse().unwrap();
    assert!(mac.is_multicast());
    assert!(!mac.is_unicast());
}

#[test]
fn test_is_broadcast() {
    let mac: MacAddr = "FF:FF:FF:FF:FF:FF".parse().unwrap();
    assert!(mac.is_broadcast());
}

#[test]
fn test_is_null() {
    let mac: MacAddr = "00:00:00:00:00:00".parse().unwrap();
    assert!(mac.is_null());
}

#[test]
fn test_is_local() {
    let mac: MacAddr = "02:00:00:00:00:00".parse().unwrap();
    assert!(mac.is_local());
    assert!(!mac.is_universal());
}

#[test]
fn test_is_universal() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert!(mac.is_universal());
    assert!(!mac.is_local());
}

// ---- OUI / NIC ----

#[test]
fn test_oui() {
    let mac: MacAddr = "00:0C:29:12:34:56".parse().unwrap();
    assert_eq!(mac.oui(), [0x00, 0x0C, 0x29]);
}

#[test]
fn test_nic() {
    let mac: MacAddr = "00:0C:29:12:34:56".parse().unwrap();
    assert_eq!(mac.nic(), [0x12, 0x34, 0x56]);
}

// ---- Arithmetic ----

#[test]
fn test_increment() {
    let mac: MacAddr = "00:00:00:00:00:01".parse().unwrap();
    assert_eq!(mac.increment(), "00:00:00:00:00:02".parse::<MacAddr>().unwrap());
}

#[test]
fn test_increment_wrap() {
    let mac: MacAddr = "00:00:00:00:00:FF".parse().unwrap();
    assert_eq!(mac.increment(), "00:00:00:00:01:00".parse::<MacAddr>().unwrap());
}

#[test]
fn test_increment_full_wrap() {
    let mac: MacAddr = "FF:FF:FF:FF:FF:FF".parse().unwrap();
    assert_eq!(mac.increment(), "00:00:00:00:00:00".parse::<MacAddr>().unwrap());
}

#[test]
fn test_decrement() {
    let mac: MacAddr = "00:00:00:00:00:02".parse().unwrap();
    assert_eq!(mac.decrement(), "00:00:00:00:00:01".parse::<MacAddr>().unwrap());
}

#[test]
fn test_decrement_wrap() {
    let mac: MacAddr = "00:00:00:00:00:00".parse().unwrap();
    assert_eq!(mac.decrement(), "FF:FF:FF:FF:FF:FF".parse::<MacAddr>().unwrap());
}

// ---- EUI-64 ----

#[test]
fn test_to_eui64() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    let eui64 = mac.to_eui64();
    assert_eq!(eui64, [0x02, 0x11, 0x22, 0xFF, 0xFE, 0x33, 0x44, 0x55]);
}

// ---- Formatting ----

#[test]
fn test_format_colon() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(mac.format_colon(), "00:11:22:33:44:55");
}

#[test]
fn test_format_hyphen() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(mac.format_hyphen(), "00-11-22-33-44-55");
}

#[test]
fn test_format_dot() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(mac.format_dot(), "0011.2233.4455");
}

#[test]
fn test_format_cisco() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(mac.format_cisco(), "0011.2233.4455");
}

#[test]
fn test_format_raw() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(mac.format_raw(), "001122334455");
}

#[test]
fn test_display() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(format!("{}", mac), "00:11:22:33:44:55");
}

// ---- MacAddrFormat ----

#[test]
fn test_format_from_str() {
    assert_eq!("colon".parse::<MacAddrFormat>().unwrap(), MacAddrFormat::Colon);
    assert_eq!("hyphen".parse::<MacAddrFormat>().unwrap(), MacAddrFormat::Hyphen);
    assert_eq!("dot".parse::<MacAddrFormat>().unwrap(), MacAddrFormat::Dot);
    assert_eq!("cisco".parse::<MacAddrFormat>().unwrap(), MacAddrFormat::Cisco);
    assert_eq!("raw".parse::<MacAddrFormat>().unwrap(), MacAddrFormat::Raw);
    assert!("invalid".parse::<MacAddrFormat>().is_err());
}

// ---- Vendor ----

#[test]
fn test_vendor_lookup() {
    let mac: MacAddr = "00:0C:29:12:34:56".parse().unwrap();
    assert_eq!(macaddr::vendor::lookup_vendor(&mac.oui()), Some("VMware, Inc."));
}

#[test]
fn test_vendor_lookup_cisco() {
    let mac: MacAddr = "00:50:B6:00:00:01".parse().unwrap();
    assert_eq!(macaddr::vendor::lookup_vendor(&mac.oui()), Some("Cisco Systems, Inc"));
}

#[test]
fn test_vendor_lookup_apple() {
    let mac: MacAddr = "08:00:07:00:00:01".parse().unwrap();
    assert_eq!(macaddr::vendor::lookup_vendor(&mac.oui()), Some("Apple, Inc."));
}

#[test]
fn test_vendor_lookup_unknown() {
    let mac: MacAddr = "00:11:22:33:44:55".parse().unwrap();
    assert_eq!(macaddr::vendor::lookup_vendor(&mac.oui()), None);
}

#[test]
fn test_list_vendors() {
    let vendors = macaddr::vendor::list_vendors();
    assert!(vendors.len() > 100);
    assert!(vendors.iter().any(|&(_, n)| n.contains("VMware")));
    assert!(vendors.iter().any(|&(_, n)| n.contains("Cisco")));
    assert!(vendors.iter().any(|&(_, n)| n.contains("Apple")));
}

// ---- Generation ----

#[test]
fn test_generate_random() {
    let mac = macaddr::vendor::generate_random();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
}

#[test]
fn test_generate_random_is_local_100_times() {
    for _ in 0..100 {
        let mac = macaddr::vendor::generate_random();
        assert!(mac.is_local(), "Random MAC should be locally administered: {}", mac);
        assert!(mac.is_unicast(), "Random MAC should be unicast: {}", mac);
    }
}

#[test]
fn test_generate_for_vendor() {
    let oui = [0x00, 0x0C, 0x29];
    let mac = macaddr::vendor::generate_for_vendor(oui);
    assert_eq!(mac.oui(), oui);
}

#[test]
fn test_generate_for_vendor_name() {
    let mac = macaddr::vendor::generate_for_vendor_name("vmware").unwrap();
    assert_eq!(mac.oui(), [0x00, 0x0C, 0x29]);
}

#[test]
fn test_generate_for_vendor_name_not_found() {
    assert!(macaddr::vendor::generate_for_vendor_name("nonexistent_vendor_xyz").is_none());
}