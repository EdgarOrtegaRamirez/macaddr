use std::fmt;
use std::str::FromStr;

/// A 6-byte MAC address (Ethernet / IEEE 802)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    pub const fn new(bytes: [u8; 6]) -> Self {
        MacAddr(bytes)
    }

    pub fn bytes(&self) -> &[u8; 6] {
        &self.0
    }

    /// Return the OUI (first 3 bytes)
    pub fn oui(&self) -> [u8; 3] {
        [self.0[0], self.0[1], self.0[2]]
    }

    /// Return the NIC-specific part (last 3 bytes)
    pub fn nic(&self) -> [u8; 3] {
        [self.0[3], self.0[4], self.0[5]]
    }

    /// Is this a unicast address? (bit 0 of first byte = 0)
    pub fn is_unicast(&self) -> bool {
        self.0[0] & 0x01 == 0
    }

    /// Is this a multicast address? (bit 0 of first byte = 1)
    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0x01 == 1
    }

    /// Is this a universally administered address? (bit 1 of first byte = 0)
    pub fn is_universal(&self) -> bool {
        self.0[0] & 0x02 == 0
    }

    /// Is this a locally administered address? (bit 1 of first byte = 1)
    pub fn is_local(&self) -> bool {
        self.0[0] & 0x02 == 2
    }

    /// Is this a broadcast address (FF:FF:FF:FF:FF:FF)?
    pub fn is_broadcast(&self) -> bool {
        self.0 == [0xFF; 6]
    }

    /// Is this a null address (00:00:00:00:00:00)?
    pub fn is_null(&self) -> bool {
        self.0 == [0x00; 6]
    }

    /// Increment the MAC address by 1
    pub fn increment(&self) -> MacAddr {
        let mut bytes = self.0;
        for i in (0..6).rev() {
            bytes[i] = bytes[i].wrapping_add(1);
            if bytes[i] != 0 {
                break;
            }
        }
        MacAddr(bytes)
    }

    /// Decrement the MAC address by 1
    pub fn decrement(&self) -> MacAddr {
        let mut bytes = self.0;
        for i in (0..6).rev() {
            bytes[i] = bytes[i].wrapping_sub(1);
            if bytes[i] != 0xFF {
                break;
            }
        }
        MacAddr(bytes)
    }

    /// Generate EUI-64 interface identifier from this MAC
    pub fn to_eui64(&self) -> [u8; 8] {
        let mut eui64 = [0u8; 8];
        eui64[0] = self.0[0] | 0x02; // Set U/L bit (bit 1)
        eui64[1] = self.0[1];
        eui64[2] = self.0[2];
        eui64[3] = 0xFF;
        eui64[4] = 0xFE;
        eui64[5] = self.0[3];
        eui64[6] = self.0[4];
        eui64[7] = self.0[5];
        eui64
    }

    /// Format as colon-separated: 00:11:22:33:44:55
    pub fn format_colon(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }

    /// Format as hyphen-separated: 00-11-22-33-44-55
    pub fn format_hyphen(&self) -> String {
        format!(
            "{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }

    /// Format as dot-separated: 0011.2233.4455
    pub fn format_dot(&self) -> String {
        format!(
            "{:02x}{:02x}.{:02x}{:02x}.{:02x}{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }

    /// Format as Cisco-style: 0011.2233.4455
    pub fn format_cisco(&self) -> String {
        self.format_dot()
    }

    /// Format with no separators: 001122334455
    pub fn format_raw(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_colon())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacAddrFormat {
    Colon,
    Hyphen,
    Dot,
    Cisco,
    Raw,
}

impl MacAddrFormat {
    pub fn all() -> Vec<MacAddrFormat> {
        vec![
            MacAddrFormat::Colon,
            MacAddrFormat::Hyphen,
            MacAddrFormat::Dot,
            MacAddrFormat::Cisco,
            MacAddrFormat::Raw,
        ]
    }
}

impl FromStr for MacAddrFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "colon" | ":" => Ok(MacAddrFormat::Colon),
            "hyphen" | "-" => Ok(MacAddrFormat::Hyphen),
            "dot" | "." => Ok(MacAddrFormat::Dot),
            "cisco" => Ok(MacAddrFormat::Cisco),
            "raw" | "none" => Ok(MacAddrFormat::Raw),
            _ => Err(format!("Unknown format: {}. Use: colon, hyphen, dot, cisco, raw", s)),
        }
    }
}

impl fmt::Display for MacAddrFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MacAddrFormat::Colon => write!(f, "colon"),
            MacAddrFormat::Hyphen => write!(f, "hyphen"),
            MacAddrFormat::Dot => write!(f, "dot"),
            MacAddrFormat::Cisco => write!(f, "cisco"),
            MacAddrFormat::Raw => write!(f, "raw"),
        }
    }
}

/// Error type for MAC address parsing
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidLength(usize),
    InvalidCharacter(char),
    InvalidFormat(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidLength(len) => write!(f, "invalid MAC address length: {} bytes (expected 6)", len),
            ParseError::InvalidCharacter(c) => write!(f, "invalid character in MAC address: '{}'", c),
            ParseError::InvalidFormat(msg) => write!(f, "invalid MAC address format: {}", msg),
        }
    }
}

impl FromStr for MacAddr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.len() == 17 && s.chars().filter(|&c| c == ':').count() == 5 {
            // Colon-separated: 00:11:22:33:44:55
            parse_delimited(s, ':')
        } else if s.len() == 17 && s.chars().filter(|&c| c == '-').count() == 5 {
            // Hyphen-separated: 00-11-22-33-44-55
            parse_delimited(s, '-')
        } else if s.len() == 14 && s.chars().filter(|&c| c == '.').count() == 2 {
            // Dot-separated: 0011.2233.4455
            let without_dots: String = s.chars().filter(|&c| c != '.').collect();
            parse_hex_bytes(&without_dots)
        } else if s.len() == 12 {
            // Raw: 001122334455
            parse_hex_bytes(s)
        } else {
            // Try to normalize: remove common separators and parse
            let cleaned: String = s.chars().filter(|&c| c != ':' && c != '-' && c != '.' && c != ' ').collect();
            if cleaned.len() == 12 {
                parse_hex_bytes(&cleaned)
            } else {
                Err(ParseError::InvalidFormat(format!(
                    "expected 6 hex bytes separated by colon/hyphen/dot or 12 hex chars, got '{}'", s
                )))
            }
        }
    }
}

fn parse_delimited(s: &str, delimiter: char) -> Result<MacAddr, ParseError> {
    let parts: Vec<&str> = s.split(delimiter).collect();
    if parts.len() != 6 {
        return Err(ParseError::InvalidFormat(format!(
            "expected 6 parts separated by '{}', got {}", delimiter, parts.len()
        )));
    }
    let mut bytes = [0u8; 6];
    for (i, part) in parts.iter().enumerate() {
        if part.len() != 2 {
            return Err(ParseError::InvalidFormat(format!(
                "expected 2 hex digits in part {}, got '{}'", i + 1, part
            )));
        }
        bytes[i] = u8::from_str_radix(part, 16)
            .map_err(|_| ParseError::InvalidCharacter(part.chars().next().unwrap_or('?')))?;
    }
    Ok(MacAddr(bytes))
}

fn parse_hex_bytes(s: &str) -> Result<MacAddr, ParseError> {
    if s.len() != 12 {
        return Err(ParseError::InvalidLength(s.len()));
    }
    let mut bytes = [0u8; 6];
    for i in 0..6 {
        let start = i * 2;
        let end = start + 2;
        bytes[i] = u8::from_str_radix(&s[start..end], 16)
            .map_err(|_| ParseError::InvalidCharacter(s.chars().nth(start).unwrap_or('?')))?;
    }
    Ok(MacAddr(bytes))
}