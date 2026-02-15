//! Utilities for parsing and formatting DNS option/parameter values.
//!
//! DNS record types such as OPT, SVCB, and HTTPS contain option/parameter
//! extensions whose values are opaque byte slices.
//! [`DnsRr::opt_datatype()`](c_ares::DnsRr::opt_datatype) indicates how to
//! interpret these bytes.  This module provides helpers to parse them into
//! meaningful Rust types and to produce human-readable representations.

use itertools::Itertools;
use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};

use c_ares::{DnsOptDataType, DnsRr, DnsRrKey};

/// Error returned when an option value cannot be parsed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptParseError(String);

impl fmt::Display for OptParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for OptParseError {}

/// A parsed DNS option value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptValue {
    /// No value.
    None,
    /// List of strings.
    StrList(Vec<String>),
    /// List of `u8` values.
    U8List(Vec<u8>),
    /// A single `u16`.
    U16(u16),
    /// List of `u16` values.
    U16List(Vec<u16>),
    /// A single `u32`.
    U32(u32),
    /// List of `u32` values.
    U32List(Vec<u32>),
    /// List of IPv4 addresses.
    InAddr4List(Vec<Ipv4Addr>),
    /// List of IPv6 addresses.
    InAddr6List(Vec<Ipv6Addr>),
    /// Opaque binary data.
    Bin(Vec<u8>),
    /// A DNS domain name.
    Name(String),
}

impl fmt::Display for OptValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptValue::None => Ok(()),
            OptValue::StrList(v) => {
                let vals = v.iter().format(", ");
                write!(f, "[{vals}]")
            }
            OptValue::U8List(v) => {
                let vals = v.iter().format(", ");
                write!(f, "[{vals}]")
            }
            OptValue::U16(v) => write!(f, "{v}"),
            OptValue::U16List(v) => {
                let vals = v.iter().format(", ");
                write!(f, "[{vals}]")
            }
            OptValue::U32(v) => write!(f, "{v}"),
            OptValue::U32List(v) => {
                let vals = v.iter().format(", ");
                write!(f, "[{vals}]")
            }
            OptValue::InAddr4List(v) => {
                let vals = v.iter().format(", ");
                write!(f, "[{vals}]")
            }
            OptValue::InAddr6List(v) => {
                let vals = v.iter().format(", ");
                write!(f, "[{vals}]")
            }
            OptValue::Bin(v) => {
                for b in v {
                    write!(f, "{b:02x}")?;
                }
                Ok(())
            }
            OptValue::Name(s) => write!(f, "{s}"),
        }
    }
}

/// Parse a raw option value according to its datatype.
///
/// Uses [`DnsRr::opt_datatype()`] to determine the wire format, then decodes
/// the byte slice into the corresponding [`OptValue`].
pub fn parse_opt_value(key: DnsRrKey, opt: u16, data: &[u8]) -> Result<OptValue, OptParseError> {
    match DnsRr::opt_datatype(key, opt) {
        DnsOptDataType::None => parse_none(data),
        DnsOptDataType::StrList => parse_str_list(data).map(OptValue::StrList),
        DnsOptDataType::U8List => Ok(OptValue::U8List(data.to_vec())),
        DnsOptDataType::U16 => parse_u16(data).map(OptValue::U16),
        DnsOptDataType::U16List => parse_u16_list(data).map(OptValue::U16List),
        DnsOptDataType::U32 => parse_u32(data).map(OptValue::U32),
        DnsOptDataType::U32List => parse_u32_list(data).map(OptValue::U32List),
        DnsOptDataType::InAddr4List => parse_inaddr4_list(data).map(OptValue::InAddr4List),
        DnsOptDataType::InAddr6List => parse_inaddr6_list(data).map(OptValue::InAddr6List),
        DnsOptDataType::Bin => Ok(OptValue::Bin(data.to_vec())),
        DnsOptDataType::Name => parse_name(data).map(OptValue::Name),
    }
}

fn parse_none(data: &[u8]) -> Result<OptValue, OptParseError> {
    if !data.is_empty() {
        return Err(OptParseError(format!(
            "expected no data, got {} bytes",
            data.len()
        )));
    }
    Ok(OptValue::None)
}

fn parse_str_list(data: &[u8]) -> Result<Vec<String>, OptParseError> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < data.len() {
        let (buf, consumed) = c_ares::expand_string(&data[pos..], data)
            .map_err(|e| OptParseError(format!("invalid string in list: {e}")))?;
        let s = std::str::from_utf8(&buf)
            .map_err(|e| OptParseError(format!("invalid UTF-8 in string list: {e}")))?;
        result.push(s.to_owned());
        pos += consumed;
    }
    if pos != data.len() {
        return Err(OptParseError(format!(
            "string list consumed {pos} bytes, but buffer is {} bytes",
            data.len()
        )));
    }
    Ok(result)
}

fn parse_u16(data: &[u8]) -> Result<u16, OptParseError> {
    data.try_into()
        .map_err(|_| OptParseError(format!("Expected 2 bytes, got {}", data.len())))
        .map(u16::from_be_bytes)
}

fn parse_u16_list(data: &[u8]) -> Result<Vec<u16>, OptParseError> {
    if !data.len().is_multiple_of(2) {
        return Err(OptParseError(format!(
            "u16 list length {} is not a multiple of 2",
            data.len()
        )));
    }

    Ok(data
        .chunks_exact(2)
        .map(|c| c.try_into().expect("chunks_exact guarantees 2 bytes"))
        .map(u16::from_be_bytes)
        .collect())
}

fn parse_u32(data: &[u8]) -> Result<u32, OptParseError> {
    data.try_into()
        .map_err(|_| OptParseError(format!("Expected 4 bytes, got {}", data.len())))
        .map(u32::from_be_bytes)
}

fn parse_u32_list(data: &[u8]) -> Result<Vec<u32>, OptParseError> {
    if !data.len().is_multiple_of(4) {
        return Err(OptParseError(format!(
            "u32 list length {} is not a multiple of 4",
            data.len()
        )));
    }

    Ok(data
        .chunks_exact(4)
        .map(|c| c.try_into().expect("chunks_exact guarantees 4 bytes"))
        .map(u32::from_be_bytes)
        .collect())
}

fn parse_inaddr4_list(data: &[u8]) -> Result<Vec<Ipv4Addr>, OptParseError> {
    if !data.len().is_multiple_of(4) {
        return Err(OptParseError(format!(
            "IPv4 address list length {} is not a multiple of 4",
            data.len()
        )));
    }

    Ok(data
        .chunks_exact(4)
        .map(|c| TryInto::<[u8; 4]>::try_into(c).expect("chunks_exact guarantees 4 bytes"))
        .map(Ipv4Addr::from)
        .collect())
}

fn parse_inaddr6_list(data: &[u8]) -> Result<Vec<Ipv6Addr>, OptParseError> {
    if !data.len().is_multiple_of(16) {
        return Err(OptParseError(format!(
            "IPv6 address list length {} is not a multiple of 16",
            data.len()
        )));
    }

    Ok(data
        .chunks_exact(16)
        .map(|c| TryInto::<[u8; 16]>::try_into(c).expect("chunks_exact guarantees 16 bytes"))
        .map(Ipv6Addr::from)
        .collect())
}

fn parse_name(data: &[u8]) -> Result<String, OptParseError> {
    let (name, consumed) = c_ares::expand_name(data, data)
        .map_err(|e| OptParseError(format!("invalid DNS name: {e}")))?;
    if consumed != data.len() {
        return Err(OptParseError(format!(
            "DNS name consumed {consumed} bytes, but buffer is {} bytes",
            data.len()
        )));
    }
    Ok(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_none_empty() {
        assert_eq!(parse_none(&[]).unwrap(), OptValue::None);
    }

    #[test]
    fn parse_none_nonempty() {
        assert!(parse_none(&[0x01]).is_err());
    }

    #[test]
    fn parse_str_list_basic() {
        // Two strings: "h3" and "h2"
        let data = [2, b'h', b'3', 2, b'h', b'2'];
        assert_eq!(
            parse_str_list(&data).unwrap(),
            vec!["h3".to_string(), "h2".to_string()]
        );
    }

    #[test]
    fn parse_str_list_invalid_utf8() {
        let data = [2, 0xff, 0xfe];
        assert!(parse_str_list(&data).is_err());
    }

    #[test]
    fn parse_str_list_truncated() {
        // Length byte says 5, but only 2 bytes follow.
        let data = [5, b'h', b'i'];
        assert!(parse_str_list(&data).is_err());
    }

    #[test]
    fn parse_str_list_trailing_data() {
        // Valid string "hi" followed by extra byte.
        let data = [2, b'h', b'i', 0xff];
        assert!(parse_str_list(&data).is_err());
    }

    #[test]
    fn parse_u16_basic() {
        assert_eq!(parse_u16(&[0x01, 0x00]).unwrap(), 256);
    }

    #[test]
    fn parse_u16_bad_length() {
        assert!(parse_u16(&[0x01]).is_err());
        assert!(parse_u16(&[0x01, 0x00, 0x00]).is_err());
    }

    #[test]
    fn parse_u16_list_basic() {
        assert_eq!(
            parse_u16_list(&[0x00, 0x01, 0x00, 0x02]).unwrap(),
            vec![1, 2]
        );
    }

    #[test]
    fn parse_u16_list_bad_length() {
        assert!(parse_u16_list(&[0x00, 0x01, 0x00]).is_err());
    }

    #[test]
    fn parse_u32_basic() {
        assert_eq!(parse_u32(&[0x00, 0x00, 0x01, 0x00]).unwrap(), 256);
    }

    #[test]
    fn parse_u32_bad_length() {
        assert!(parse_u32(&[0x00, 0x00, 0x01]).is_err());
        assert!(parse_u32(&[0x00, 0x00, 0x01, 0x00, 0x00]).is_err());
    }

    #[test]
    fn parse_u32_list_bad_length() {
        assert!(parse_u32_list(&[0x00, 0x00, 0x01]).is_err());
    }

    #[test]
    fn parse_inaddr4_list_basic() {
        let data = [127, 0, 0, 1, 10, 0, 0, 1];
        assert_eq!(
            parse_inaddr4_list(&data).unwrap(),
            vec![Ipv4Addr::new(127, 0, 0, 1), Ipv4Addr::new(10, 0, 0, 1),]
        );
    }

    #[test]
    fn parse_inaddr4_list_bad_length() {
        assert!(parse_inaddr4_list(&[1, 2, 3]).is_err());
    }

    #[test]
    fn parse_inaddr6_list_basic() {
        let mut data = [0u8; 16];
        data[0] = 0x20;
        data[1] = 0x01;
        let result = parse_inaddr6_list(&data).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].octets()[0], 0x20);
    }

    #[test]
    fn parse_inaddr6_list_bad_length() {
        assert!(parse_inaddr6_list(&[0u8; 15]).is_err());
    }

    #[test]
    fn parse_name_basic() {
        // "www.example.com" in DNS wire format.
        let data = [
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0,
        ];
        assert_eq!(parse_name(&data).unwrap(), "www.example.com");
    }

    #[test]
    fn parse_name_truncated() {
        // Label length says 5, but only 2 bytes follow before terminator.
        let data = [5, b'a', b'b', 0];
        assert!(parse_name(&data).is_err());
    }

    #[test]
    fn parse_name_trailing_data() {
        // Valid name "com." followed by extra bytes.
        let data = [3, b'c', b'o', b'm', 0, 0xff];
        assert!(parse_name(&data).is_err());
    }

    #[test]
    fn format_opt_value_display() {
        let val = OptValue::InAddr4List(vec![Ipv4Addr::new(1, 2, 3, 4)]);
        assert_eq!(val.to_string(), "[1.2.3.4]");
    }

    #[test]
    fn format_opt_value_bin() {
        let val = OptValue::Bin(vec![0xca, 0xfe]);
        assert_eq!(val.to_string(), "cafe");
    }

    #[test]
    fn display_none() {
        assert_eq!(OptValue::None.to_string(), "");
    }

    #[test]
    fn display_str_list() {
        let val = OptValue::StrList(vec!["h3".to_owned(), "h2".to_owned()]);
        assert_eq!(val.to_string(), "[h3, h2]");
    }

    #[test]
    fn display_u8_list() {
        let val = OptValue::U8List(vec![1, 2, 3]);
        assert_eq!(val.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn display_u16() {
        assert_eq!(OptValue::U16(42).to_string(), "42");
    }

    #[test]
    fn display_u16_list() {
        let val = OptValue::U16List(vec![100, 200]);
        assert_eq!(val.to_string(), "[100, 200]");
    }

    #[test]
    fn display_u32() {
        assert_eq!(OptValue::U32(123456).to_string(), "123456");
    }

    #[test]
    fn display_u32_list() {
        let val = OptValue::U32List(vec![1, 2]);
        assert_eq!(val.to_string(), "[1, 2]");
    }

    #[test]
    fn display_inaddr6_list() {
        let val = OptValue::InAddr6List(vec![Ipv6Addr::LOCALHOST]);
        assert_eq!(val.to_string(), "[::1]");
    }

    #[test]
    fn display_name() {
        let val = OptValue::Name("example.com".to_owned());
        assert_eq!(val.to_string(), "example.com");
    }

    #[test]
    fn display_bin_empty() {
        assert_eq!(OptValue::Bin(vec![]).to_string(), "");
    }

    #[test]
    fn opt_parse_error_display() {
        let err = OptParseError("test error".to_owned());
        assert_eq!(err.to_string(), "test error");
    }

    #[test]
    fn opt_parse_error_is_error() {
        let err = OptParseError("test".to_owned());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn parse_str_list_empty() {
        assert_eq!(parse_str_list(&[]).unwrap(), Vec::<String>::new());
    }

    #[test]
    fn parse_u16_list_empty() {
        assert_eq!(parse_u16_list(&[]).unwrap(), Vec::<u16>::new());
    }

    #[test]
    fn parse_u32_list_basic() {
        assert_eq!(
            parse_u32_list(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]).unwrap(),
            vec![1, 2]
        );
    }

    #[test]
    fn parse_u32_list_empty() {
        assert_eq!(parse_u32_list(&[]).unwrap(), Vec::<u32>::new());
    }

    #[test]
    fn parse_inaddr4_list_empty() {
        assert_eq!(parse_inaddr4_list(&[]).unwrap(), Vec::<Ipv4Addr>::new());
    }

    #[test]
    fn parse_inaddr6_list_empty() {
        assert_eq!(parse_inaddr6_list(&[]).unwrap(), Vec::<Ipv6Addr>::new());
    }

    #[test]
    fn parse_inaddr6_list_multiple() {
        let mut data = [0u8; 32];
        // ::1
        data[15] = 1;
        // ::2
        data[31] = 2;
        let result = parse_inaddr6_list(&data).unwrap();
        assert_eq!(
            result,
            vec![Ipv6Addr::LOCALHOST, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 2)]
        );
    }

    // --- parse_opt_value integration tests ---
    // These use HTTPS_PARAMS opt codes with known datatype mappings.

    #[test]
    fn parse_opt_value_none() {
        // HTTPS param 2 = no-default-alpn (None)
        let result = parse_opt_value(DnsRrKey::HTTPS_PARAMS, 2, &[]).unwrap();
        assert_eq!(result, OptValue::None);
    }

    #[test]
    fn parse_opt_value_none_rejects_data() {
        assert!(parse_opt_value(DnsRrKey::HTTPS_PARAMS, 2, &[0x01]).is_err());
    }

    #[test]
    fn parse_opt_value_str_list() {
        // HTTPS param 1 = alpn (StrList)
        let data = [2, b'h', b'3'];
        let result = parse_opt_value(DnsRrKey::HTTPS_PARAMS, 1, &data).unwrap();
        assert_eq!(result, OptValue::StrList(vec!["h3".to_owned()]));
    }

    #[test]
    fn parse_opt_value_u16() {
        // HTTPS param 3 = port (U16)
        let result = parse_opt_value(DnsRrKey::HTTPS_PARAMS, 3, &[0x01, 0xBB]).unwrap();
        assert_eq!(result, OptValue::U16(443));
    }

    #[test]
    fn parse_opt_value_u16_bad_length() {
        assert!(parse_opt_value(DnsRrKey::HTTPS_PARAMS, 3, &[0x01]).is_err());
    }

    #[test]
    fn parse_opt_value_u16_list() {
        // HTTPS param 0 = mandatory (U16List)
        let result = parse_opt_value(DnsRrKey::HTTPS_PARAMS, 0, &[0x00, 0x01, 0x00, 0x03]).unwrap();
        assert_eq!(result, OptValue::U16List(vec![1, 3]));
    }

    #[test]
    fn parse_opt_value_inaddr4_list() {
        // HTTPS param 4 = ipv4hint (InAddr4List)
        let result = parse_opt_value(DnsRrKey::HTTPS_PARAMS, 4, &[127, 0, 0, 1]).unwrap();
        assert_eq!(
            result,
            OptValue::InAddr4List(vec![Ipv4Addr::new(127, 0, 0, 1)])
        );
    }

    #[test]
    fn parse_opt_value_inaddr4_list_bad_length() {
        assert!(parse_opt_value(DnsRrKey::HTTPS_PARAMS, 4, &[1, 2, 3]).is_err());
    }

    #[test]
    fn parse_opt_value_bin() {
        // HTTPS param 5 = ech (Bin)
        let result = parse_opt_value(DnsRrKey::HTTPS_PARAMS, 5, &[0xca, 0xfe]).unwrap();
        assert_eq!(result, OptValue::Bin(vec![0xca, 0xfe]));
    }

    #[test]
    fn parse_opt_value_inaddr6_list() {
        // HTTPS param 6 = ipv6hint (InAddr6List)
        let mut data = [0u8; 16];
        data[15] = 1;
        let result = parse_opt_value(DnsRrKey::HTTPS_PARAMS, 6, &data).unwrap();
        assert_eq!(result, OptValue::InAddr6List(vec![Ipv6Addr::LOCALHOST]));
    }

    #[test]
    fn parse_opt_value_inaddr6_list_bad_length() {
        assert!(parse_opt_value(DnsRrKey::HTTPS_PARAMS, 6, &[0u8; 15]).is_err());
    }

    #[test]
    fn parse_opt_value_u32() {
        // OPT option 2 = UL (U32)
        let result = parse_opt_value(DnsRrKey::OPT_OPTIONS, 2, &[0x00, 0x01, 0x51, 0x80]).unwrap();
        assert_eq!(result, OptValue::U32(86400));
    }

    #[test]
    fn parse_opt_value_u32_bad_length() {
        assert!(parse_opt_value(DnsRrKey::OPT_OPTIONS, 2, &[0x00, 0x01, 0x51]).is_err());
    }

    #[test]
    fn parse_opt_value_u8_list() {
        // OPT option 5 = DAU (U8List)
        let result = parse_opt_value(DnsRrKey::OPT_OPTIONS, 5, &[8, 13, 14]).unwrap();
        assert_eq!(result, OptValue::U8List(vec![8, 13, 14]));
    }

    #[test]
    fn parse_opt_value_name() {
        // OPT option 13 = CHAIN (Name)
        let data = [
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        let result = parse_opt_value(DnsRrKey::OPT_OPTIONS, 13, &data).unwrap();
        assert_eq!(result, OptValue::Name("example.com".to_owned()));
    }
}
