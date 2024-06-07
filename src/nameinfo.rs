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
        Self {
            node: result.node().map(std::borrow::ToOwned::to_owned),
            service: result.service().map(std::borrow::ToOwned::to_owned),
        }
    }
}
