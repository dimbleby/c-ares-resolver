/// An owned version of `c_ares::NameInfoResult`.
#[derive(Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct NameInfoResult {
    /// The node returned by the lookup.
    pub node: Option<String>,

    /// The service returned by the lookup.
    pub service: Option<String>,
}

impl From<c_ares::NameInfoResult<'_>> for NameInfoResult {
    fn from(result: c_ares::NameInfoResult) -> Self {
        Self {
            node: result.node().map(std::borrow::ToOwned::to_owned),
            service: result.service().map(std::borrow::ToOwned::to_owned),
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
    fn name_info_result_is_send() {
        assert_send::<NameInfoResult>();
    }

    #[test]
    fn name_info_result_is_sync() {
        assert_sync::<NameInfoResult>();
    }

    #[test]
    fn name_info_result_clone() {
        let result = NameInfoResult {
            node: Some("example.com".to_string()),
            service: Some("http".to_string()),
        };
        let cloned = result.clone();
        assert_eq!(result, cloned);
    }

    #[test]
    fn name_info_result_eq() {
        let result1 = NameInfoResult {
            node: Some("example.com".to_string()),
            service: None,
        };
        let result2 = NameInfoResult {
            node: Some("example.com".to_string()),
            service: None,
        };
        assert_eq!(result1, result2);
    }

    #[test]
    fn name_info_result_ne() {
        let result1 = NameInfoResult {
            node: Some("example.com".to_string()),
            service: None,
        };
        let result2 = NameInfoResult {
            node: Some("other.com".to_string()),
            service: None,
        };
        assert_ne!(result1, result2);
    }

    #[test]
    fn name_info_result_debug() {
        let result = NameInfoResult {
            node: Some("example.com".to_string()),
            service: Some("https".to_string()),
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("example.com"));
        assert!(debug.contains("https"));
    }

    #[test]
    fn name_info_result_hash() {
        let result = NameInfoResult {
            node: Some("example.com".to_string()),
            service: None,
        };
        let mut hasher = DefaultHasher::new();
        result.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        result.clone().hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn name_info_result_ord() {
        let result1 = NameInfoResult {
            node: Some("a.com".to_string()),
            service: None,
        };
        let result2 = NameInfoResult {
            node: Some("b.com".to_string()),
            service: None,
        };
        assert!(result1 < result2);
    }

    #[test]
    fn name_info_result_none_values() {
        let result = NameInfoResult {
            node: None,
            service: None,
        };
        assert!(result.node.is_none());
        assert!(result.service.is_none());
    }
}
