use std::net::IpAddr;

use c_ares;

/// An owned version of `c_ares::HostResults`.
#[derive(Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct HostResults{
    hostname: String,
    addresses: Vec<IpAddr>,
    aliases: Vec<String>,
}

impl<'a> From<c_ares::HostResults<'a>> for HostResults {
    fn from(results: c_ares::HostResults) -> Self {
        HostResults {
            hostname: results.hostname().to_owned(),
            addresses: results.addresses().collect(),
            aliases: results.aliases().map(|s| s.to_owned()).collect(),
        }
    }
}
