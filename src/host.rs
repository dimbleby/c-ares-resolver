use std::net::IpAddr;

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

impl From<c_ares::HostResults<'_>> for HostResults {
    fn from(results: c_ares::HostResults) -> Self {
        Self {
            hostname: results.hostname().to_owned(),
            addresses: results.addresses().collect(),
            aliases: results
                .aliases()
                .map(std::borrow::ToOwned::to_owned)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn host_results_is_send() {
        assert_send::<HostResults>();
    }

    #[test]
    fn host_results_is_sync() {
        assert_sync::<HostResults>();
    }

    #[test]
    fn host_results_clone() {
        let results = HostResults {
            hostname: "example.com".to_string(),
            addresses: vec!["127.0.0.1".parse().unwrap()],
            aliases: vec!["alias.example.com".to_string()],
        };
        let cloned = results.clone();
        assert_eq!(results, cloned);
    }

    #[test]
    fn host_results_eq() {
        let results1 = HostResults {
            hostname: "example.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        let results2 = HostResults {
            hostname: "example.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        assert_eq!(results1, results2);
    }

    #[test]
    fn host_results_ne() {
        let results1 = HostResults {
            hostname: "example.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        let results2 = HostResults {
            hostname: "other.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        assert_ne!(results1, results2);
    }

    #[test]
    fn host_results_debug() {
        let results = HostResults {
            hostname: "example.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        let debug = format!("{:?}", results);
        assert!(debug.contains("example.com"));
    }

    #[test]
    fn host_results_hash() {
        let results = HostResults {
            hostname: "example.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        let mut hasher = DefaultHasher::new();
        results.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        results.clone().hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn host_results_ord() {
        let results1 = HostResults {
            hostname: "a.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        let results2 = HostResults {
            hostname: "b.com".to_string(),
            addresses: vec![],
            aliases: vec![],
        };
        assert!(results1 < results2);
    }

    #[test]
    fn host_results_with_addresses() {
        let results = HostResults {
            hostname: "example.com".to_string(),
            addresses: vec!["127.0.0.1".parse().unwrap(), "::1".parse().unwrap()],
            aliases: vec!["www.example.com".to_string()],
        };
        assert_eq!(results.addresses.len(), 2);
        assert_eq!(results.aliases.len(), 1);
    }
}
