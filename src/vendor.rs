use rand::Rng;

use crate::models::MacAddr;

/// Built-in OUI database (common vendors)
static BUILTIN_OUIS: &[(&str, &str)] = &[
    ("00037F", "Intel Corporation"),
    ("000C29", "VMware, Inc."),
    ("00155D", "Microsoft Corporation"),
    ("001C42", "Parallels, Inc."),
    ("0021E8", "Broadcom Limited"),
    ("002268", "Dell Inc."),
    ("002590", "Apple, Inc."),
    ("003048", "Dell Inc."),
    ("0050B6", "Cisco Systems, Inc"),
    ("008037", "Intel Corporation"),
    ("00903C", "Google, Inc."),
    ("00904F", "Nokia Corporation"),
    ("00A0C9", "Intel Corporation"),
    ("00AA01", "Hewlett-Packard Company"),
    ("00D0B7", "Oracle Corporation"),
    ("080007", "Apple, Inc."),
    ("080020", "Sun Microsystems"),
    ("08EDB9", "Oculus VR, Inc."),
    ("0A0027", "Apple, Inc."),
    ("10D5A1", "Shenzhen Four Seas Global Link Network Technology Co., Ltd."),
    ("10E8EE", "Huawei Technologies Co., Ltd"),
    ("18B430", "Samsung Electronics Co., Ltd."),
    ("1C1B0D", "Aruba Networks"),
    ("1C69A5", "Netgear Inc."),
    ("2462AB", "D-Link Corporation"),
    ("28C02C", "LG Electronics Inc."),
    ("2C27D7", "Google, Inc."),
    ("2C542D", "Ubiquiti Inc."),
    ("30AEF6", "Hewlett Packard Enterprise"),
    ("3842A6", "Raspberry Pi Foundation"),
    ("3C5E8B", "Ubiquiti Networks Inc."),
    ("3CE5A6", "Hewlett Packard Enterprise"),
    ("40A8F0", "Zyxel Communications Corporation"),
    ("44D9E8", "AWS/Amazon.com"),
    ("482AE3", "AWS/Amazon.com"),
    ("485AB6", "Huawei Technologies Co., Ltd"),
    ("48F8B3", "Cisco Systems, Inc"),
    ("4C5F70", "ASUSTek Computer Inc."),
    ("508569", "Intel Corporate"),
    ("508A06", "Roku, Inc."),
    ("50D274", "Samsung Electronics Co., Ltd."),
    ("549F35", "Netgear Inc."),
    ("585076", "TP-LINK TECHNOLOGIES CO., LTD."),
    ("5CF3FC", "IBM Corp"),
    ("60F189", "Apple, Inc."),
    ("6465C0", "Kyocera Corporation"),
    ("6476BA", "Ubiquiti Networks Inc."),
    ("68A86D", "Hewlett Packard"),
    ("6C2E85", "Samsung Electronics"),
    ("6C3E6D", "Amazon Technologies Inc."),
    ("6C8814", "Apple, Inc."),
    ("70B3D5", "Cisco Systems Inc."),
    ("74DA38", "Roku, Inc."),
    ("78A3E4", "Samsung Electronics Co., Ltd."),
    ("78D6F0", "Netgear"),
    ("7C11BE", "Apple, Inc."),
    ("7CF043", "Ubiquiti Inc."),
    ("803773", "Samsung Electronics Co., Ltd"),
    ("80B03C", "D-Link Corporation"),
    ("8416F9", "Google, Inc."),
    ("846EB0", "Amazon Technologies Inc."),
    ("883FD3", "Fortinet Inc."),
    ("8891DD", "Apple, Inc."),
    ("8C8590", "Cisco Systems, Inc"),
    ("8C8E76", "Samsung Electronics Co., Ltd."),
    ("90A4DE", "D-Link International"),
    ("90E6BA", "Huawei Technologies Co., Ltd"),
    ("94B8C6", "AWS/Amazon.com"),
    ("98D6F7", "Dell Inc."),
    ("9C5C8E", "Google Inc."),
    ("9C6B00", "Hewlett Packard Enterprise"),
    ("A0369F", "Cisco Systems, Inc"),
    ("A4ADB8", "Cisco Systems, Inc"),
    ("ACBC32", "Azurewave Technologies, Inc."),
    ("B0A10A", "Nest Labs Inc."),
    ("B0E235", "Ubiquiti Networks Inc."),
    ("BC7670", "TP-LINK TECHNOLOGIES CO., LTD."),
    ("BCFC5A", "Palo Alto Networks"),
    ("C02B7C", "Cisco Systems, Inc"),
    ("C4024A", "Samsung Electronics Co., Ltd."),
    ("C8CBB8", "HTC Corporation"),
    ("CC2DE0", "Google, Inc."),
    ("CC34D1", "Lenovo Group Limited"),
    ("CC78AB", "Sonos, Inc."),
    ("D0034B", "Intel Corporate"),
    ("D0A623", "HP Inc."),
    ("D423E4", "Arris Group Inc."),
    ("D888CE", "Intel Corporate"),
    ("D8B377", "Huawei Technologies Co., Ltd"),
    ("DC2B2A", "HTC Corporation"),
    ("DC4F22", "Check Point Software Technologies"),
    ("DCA632", "Palo Alto Networks"),
    ("E008D5", "Google, Inc."),
    ("E0ACCB", "ASUSTek Computer Inc."),
    ("E41F13", "Raspberry Pi Foundation"),
    ("E4956E", "Apple, Inc."),
    ("E8F1B0", "Oracle Corporation"),
    ("F01C2D", "Liteon Technology Corporation"),
    ("F01898", "Samsung Electronics Co., Ltd."),
    ("F07960", "Google, Inc."),
    ("F0B429", "Texas Instruments"),
    ("F48E92", "SFR"),
    ("F8E71E", "ASUSTek Computer Inc."),
    ("FCF152", "Hewlett Packard"),
    ("FCFBFB", "Cisco Systems, Inc"),
];

/// Look up vendor by OUI
pub fn lookup_vendor(oui: &[u8; 3]) -> Option<&'static str> {
    let hex = format!("{:02X}{:02X}{:02X}", oui[0], oui[1], oui[2]);
    BUILTIN_OUIS.iter()
        .find(|&&(key, _)| key == hex)
        .map(|&(_, vendor)| vendor)
}

/// Generate a random MAC address
pub fn generate_random() -> MacAddr {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 6];
    rng.fill(&mut bytes);
    // Clear multicast bit, set locally administered bit for random
    bytes[0] = (bytes[0] & 0xFE) | 0x02;
    MacAddr::new(bytes)
}

/// Generate a MAC from a specific vendor OUI
pub fn generate_for_vendor(oui: [u8; 3]) -> MacAddr {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 6];
    bytes[0..3].copy_from_slice(&oui);
    rng.fill(&mut bytes[3..6]);
    MacAddr::new(bytes)
}

/// Generate a MAC from a vendor name (random pick)
pub fn generate_for_vendor_name(name: &str) -> Option<MacAddr> {
    let name_lower = name.to_lowercase();
    let matched = BUILTIN_OUIS.iter().find(|&&(_, v)| v.to_lowercase().contains(&name_lower))?;
    let hex = matched.0;
    let oui = [
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ];
    Some(generate_for_vendor(oui))
}

/// List all known vendors
pub fn list_vendors() -> Vec<(&'static str, &'static str)> {
    BUILTIN_OUIS.to_vec()
}
