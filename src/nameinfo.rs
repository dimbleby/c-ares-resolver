use c_ares;

/// An owned version of `c_ares::NameInfoResult`.
#[derive(Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct NameInfoResult {
    /// The node returned by the lookup.
    pub node: Option<String>,

    /// The service returned by the lookup.
    pub service: Option<String>,
}

impl<'a> From<c_ares::NameInfoResult<'a>> for NameInfoResult {
    fn from(result: c_ares::NameInfoResult) -> Self {
        NameInfoResult {
            node: result.node().map(|x| x.to_owned()),
            service: result.service().map(|x| x.to_owned()),
        }
    }
}
