use std::net::IpAddr;

use c_ares;

/// An owned version of `c_ares::HostResults`.
#[derive(Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct HostResults {
    /// The hostname returned by the lookup.
    pub hostname: String,

    /// The IP addresses returned by the lookup.
    pub addresses: Vec<IpAddr>,

    /// The aliases returned by the lookup.
    pub aliases: Vec<String>,
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
