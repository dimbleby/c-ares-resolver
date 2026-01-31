use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use crate::error::Error;
use crate::host::HostResults;
use crate::nameinfo::NameInfoResult;
use crate::resolver::{Options, Resolver};

#[cfg(cares1_24)]
use c_ares::AresString;

#[cfg(cares1_29)]
use c_ares::ServerStateFlags;

use std::sync::mpsc;

/// A blocking DNS resolver.
pub struct BlockingResolver {
    inner: Resolver,
}

// Most query implementations follow the same pattern: call through to the `Resolver`, arranging
// that the callback sends the result down a channel.
macro_rules! blockify {
    ($resolver:expr, $query:ident, $question:expr) => {{
        let (tx, rx) = mpsc::sync_channel(1);
        $resolver.$query($question, move |result| tx.send(result).unwrap());
        rx.recv().unwrap()
    }};
}

impl BlockingResolver {
    /// Create a new `BlockingResolver`, using default `Options`.
    pub fn new() -> Result<Self, Error> {
        let options = Options::default();
        Self::with_options(options)
    }

    /// Create a new `BlockingResolver`, with the given `Options`.
    pub fn with_options(options: Options) -> Result<Self, Error> {
        let inner = Resolver::with_options(options)?;
        let resolver = Self { inner };
        Ok(resolver)
    }

    /// Reinitialize a channel from system configuration.
    #[cfg(cares1_22)]
    pub fn reinit(&self) -> c_ares::Result<&Self> {
        self.inner.reinit()?;
        Ok(self)
    }

    /// Set the list of servers to contact, instead of the servers specified in resolv.conf or the
    /// local named.
    ///
    /// String format is `host[:port]`.  IPv6 addresses with ports require square brackets eg
    /// `[2001:4860:4860::8888]:53`.
    pub fn set_servers(&self, servers: &[&str]) -> c_ares::Result<&Self> {
        self.inner.set_servers(servers)?;
        Ok(self)
    }

    /// Retrieves the list of servers in comma delimited format.
    #[cfg(cares1_24)]
    pub fn get_servers(&self) -> AresString {
        self.inner.get_servers()
    }

    /// Set the local IPv4 address from which to make queries.
    pub fn set_local_ipv4(&self, ipv4: Ipv4Addr) -> &Self {
        self.inner.set_local_ipv4(ipv4);
        self
    }

    /// Set the local IPv6 address from which to make queries.
    pub fn set_local_ipv6(&self, ipv6: &Ipv6Addr) -> &Self {
        self.inner.set_local_ipv6(ipv6);
        self
    }

    /// Set the local device from which to make queries.
    pub fn set_local_device(&self, device: &str) -> &Self {
        self.inner.set_local_device(device);
        self
    }

    /// Initializes an address sortlist configuration, so that addresses returned by
    /// `get_host_by_name()` are sorted according to the sortlist.
    ///
    /// Each element of the sortlist holds an IP-address/netmask pair. The netmask is optional but
    /// follows the address after a slash if present. For example: "130.155.160.0/255.255.240.0",
    /// or "130.155.0.0".
    pub fn set_sortlist(&self, sortlist: &[&str]) -> c_ares::Result<&Self> {
        self.inner.set_sortlist(sortlist)?;
        Ok(self)
    }

    /// Set a callback function to be invoked whenever a query on the channel completes.
    ///
    /// `callback(server, success, flags)` will be called when a query completes.
    ///
    /// - `server` indicates the DNS server that was used for the query.
    /// - `success` indicates whether the query succeeded or not.
    /// - `flags` is a bitmask of flags describing various aspects of the query.
    #[cfg(cares1_29)]
    pub fn set_server_state_callback<F>(&self, callback: F) -> &Self
    where
        F: FnMut(&str, bool, ServerStateFlags) + Send + 'static,
    {
        self.inner.set_server_state_callback(callback);
        self
    }

    /// Look up the A records associated with `name`.
    pub fn query_a(&self, name: &str) -> c_ares::Result<c_ares::AResults> {
        blockify!(self.inner, query_a, name)
    }

    /// Search for the A records associated with `name`.
    pub fn search_a(&self, name: &str) -> c_ares::Result<c_ares::AResults> {
        blockify!(self.inner, search_a, name)
    }

    /// Look up the AAAA records associated with `name`.
    pub fn query_aaaa(&self, name: &str) -> c_ares::Result<c_ares::AAAAResults> {
        blockify!(self.inner, query_aaaa, name)
    }

    /// Search for the AAAA records associated with `name`.
    pub fn search_aaaa(&self, name: &str) -> c_ares::Result<c_ares::AAAAResults> {
        blockify!(self.inner, search_aaaa, name)
    }

    /// Look up the CAA records associated with `name`.
    pub fn query_caa(&self, name: &str) -> c_ares::Result<c_ares::CAAResults> {
        blockify!(self.inner, query_caa, name)
    }

    /// Search for the CAA records associated with `name`.
    pub fn search_caa(&self, name: &str) -> c_ares::Result<c_ares::CAAResults> {
        blockify!(self.inner, search_caa, name)
    }

    /// Look up the CNAME records associated with `name`.
    pub fn query_cname(&self, name: &str) -> c_ares::Result<c_ares::CNameResults> {
        blockify!(self.inner, query_cname, name)
    }

    /// Search for the CNAME records associated with `name`.
    pub fn search_cname(&self, name: &str) -> c_ares::Result<c_ares::CNameResults> {
        blockify!(self.inner, search_cname, name)
    }

    /// Look up the MX records associated with `name`.
    pub fn query_mx(&self, name: &str) -> c_ares::Result<c_ares::MXResults> {
        blockify!(self.inner, query_mx, name)
    }

    /// Search for the MX records associated with `name`.
    pub fn search_mx(&self, name: &str) -> c_ares::Result<c_ares::MXResults> {
        blockify!(self.inner, search_mx, name)
    }

    /// Look up the NAPTR records associated with `name`.
    pub fn query_naptr(&self, name: &str) -> c_ares::Result<c_ares::NAPTRResults> {
        blockify!(self.inner, query_naptr, name)
    }

    /// Search for the NAPTR records associated with `name`.
    pub fn search_naptr(&self, name: &str) -> c_ares::Result<c_ares::NAPTRResults> {
        blockify!(self.inner, search_naptr, name)
    }

    /// Look up the NS records associated with `name`.
    pub fn query_ns(&self, name: &str) -> c_ares::Result<c_ares::NSResults> {
        blockify!(self.inner, query_ns, name)
    }

    /// Search for the NS records associated with `name`.
    pub fn search_ns(&self, name: &str) -> c_ares::Result<c_ares::NSResults> {
        blockify!(self.inner, search_ns, name)
    }

    /// Look up the PTR records associated with `name`.
    pub fn query_ptr(&self, name: &str) -> c_ares::Result<c_ares::PTRResults> {
        blockify!(self.inner, query_ptr, name)
    }

    /// Search for the PTR records associated with `name`.
    pub fn search_ptr(&self, name: &str) -> c_ares::Result<c_ares::PTRResults> {
        blockify!(self.inner, search_ptr, name)
    }

    /// Look up the SOA records associated with `name`.
    pub fn query_soa(&self, name: &str) -> c_ares::Result<c_ares::SOAResult> {
        blockify!(self.inner, query_soa, name)
    }

    /// Search for the SOA records associated with `name`.
    pub fn search_soa(&self, name: &str) -> c_ares::Result<c_ares::SOAResult> {
        blockify!(self.inner, search_soa, name)
    }

    /// Look up the SRV records associated with `name`.
    pub fn query_srv(&self, name: &str) -> c_ares::Result<c_ares::SRVResults> {
        blockify!(self.inner, query_srv, name)
    }

    /// Search for the SRV records associated with `name`.
    pub fn search_srv(&self, name: &str) -> c_ares::Result<c_ares::SRVResults> {
        blockify!(self.inner, search_srv, name)
    }

    /// Look up the TXT records associated with `name`.
    pub fn query_txt(&self, name: &str) -> c_ares::Result<c_ares::TXTResults> {
        blockify!(self.inner, query_txt, name)
    }

    /// Search for the TXT records associated with `name`.
    pub fn search_txt(&self, name: &str) -> c_ares::Result<c_ares::TXTResults> {
        blockify!(self.inner, search_txt, name)
    }

    /// Look up the URI records associated with `name`.
    pub fn query_uri(&self, name: &str) -> c_ares::Result<c_ares::URIResults> {
        blockify!(self.inner, query_uri, name)
    }

    /// Search for the URI records associated with `name`.
    pub fn search_uri(&self, name: &str) -> c_ares::Result<c_ares::URIResults> {
        blockify!(self.inner, search_uri, name)
    }

    /// Perform a host query by address.
    ///
    /// This method is one of the very few places where this library performs strictly more
    /// allocation than the underlying `c-ares` code.  If this is a problem for you, you should
    /// prefer to use the analogous method on the `Resolver`.
    pub fn get_host_by_address(&self, address: &IpAddr) -> c_ares::Result<HostResults> {
        let (tx, rx) = mpsc::sync_channel(1);
        self.inner.get_host_by_address(address, move |result| {
            tx.send(result.map(Into::into)).unwrap()
        });
        rx.recv().unwrap()
    }

    /// Perform a host query by name.
    ///
    /// This method is one of the very few places where this library performs strictly more
    /// allocation than the underlying `c-ares` code.  If this is a problem for you, you should
    /// prefer to use the analogous method on the `Resolver`.
    pub fn get_host_by_name(
        &self,
        name: &str,
        family: c_ares::AddressFamily,
    ) -> c_ares::Result<HostResults> {
        let (tx, rx) = mpsc::sync_channel(1);
        self.inner.get_host_by_name(name, family, move |result| {
            tx.send(result.map(Into::into)).unwrap()
        });
        rx.recv().unwrap()
    }

    /// Address-to-nodename translation in protocol-independent manner.
    ///
    /// This method is one of the very few places where this library performs strictly more
    /// allocation than the underlying `c-ares` code.  If this is a problem for you, you should
    /// prefer to use the analogous method on the `Resolver`.
    pub fn get_name_info(
        &self,
        address: &SocketAddr,
        flags: c_ares::NIFlags,
    ) -> c_ares::Result<NameInfoResult> {
        let (tx, rx) = mpsc::sync_channel(1);
        self.inner.get_name_info(address, flags, move |result| {
            tx.send(result.map(Into::into)).unwrap()
        });
        rx.recv().unwrap()
    }

    /// Initiate a single-question DNS query for `name`.  The class and type of the query are per
    /// the provided parameters, taking values as defined in `arpa/nameser.h`.
    ///
    /// This method is one of the very few places where this library performs strictly more
    /// allocation than the underlying `c-ares` code.  If this is a problem for you, you should
    /// prefer to use the analogous method on the `Resolver`.
    ///
    /// This method is provided so that users can query DNS types for which `c-ares` does not
    /// provide a parser; or in case a third-party parser is preferred.  Usually, if a suitable
    /// `query_xxx()` is available, that should be used.
    pub fn query(&self, name: &str, dns_class: u16, query_type: u16) -> c_ares::Result<Vec<u8>> {
        let (tx, rx) = mpsc::sync_channel(1);
        self.inner
            .query(name, dns_class, query_type, move |result| {
                tx.send(result.map(std::borrow::ToOwned::to_owned)).unwrap()
            });
        rx.recv().unwrap()
    }

    /// Initiate a series of single-question DNS queries for `name`.  The class and type of the
    /// query are per the provided parameters, taking values as defined in `arpa/nameser.h`.
    ///
    /// This method is one of the very few places where this library performs strictly more
    /// allocation than the underlying `c-ares` code.  If this is a problem for you, you should
    /// prefer to use the analogous method on the `Resolver`.
    ///
    /// This method is provided so that users can search DNS types for which `c-ares` does not
    /// provide a parser; or in case a third-party parser is preferred.  Usually, if a suitable
    /// `search_xxx()` is available, that should be used.
    pub fn search(&self, name: &str, dns_class: u16, query_type: u16) -> c_ares::Result<Vec<u8>> {
        let (tx, rx) = mpsc::sync_channel(1);
        self.inner
            .search(name, dns_class, query_type, move |result| {
                tx.send(result.map(std::borrow::ToOwned::to_owned)).unwrap()
            });
        rx.recv().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn blocking_resolver_is_send() {
        assert_send::<BlockingResolver>();
    }

    #[test]
    fn blocking_resolver_is_sync() {
        assert_sync::<BlockingResolver>();
    }

    #[test]
    fn blocking_resolver_new() {
        let resolver = BlockingResolver::new();
        assert!(resolver.is_ok());
    }

    #[test]
    fn blocking_resolver_with_options() {
        let options = Options::new();
        let resolver = BlockingResolver::with_options(options);
        assert!(resolver.is_ok());
    }

    #[test]
    fn blocking_resolver_with_custom_options() {
        let mut options = Options::new();
        options.set_timeout(2000).set_tries(2);
        let resolver = BlockingResolver::with_options(options);
        assert!(resolver.is_ok());
    }

    #[test]
    fn blocking_resolver_set_local_ipv4() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.set_local_ipv4(Ipv4Addr::new(127, 0, 0, 1));
        assert!(std::ptr::eq(result, &resolver));
    }

    #[test]
    fn blocking_resolver_set_local_ipv6() {
        let resolver = BlockingResolver::new().unwrap();
        let ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        let result = resolver.set_local_ipv6(&ipv6);
        assert!(std::ptr::eq(result, &resolver));
    }

    #[test]
    fn blocking_resolver_set_local_device() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.set_local_device("lo");
        assert!(std::ptr::eq(result, &resolver));
    }

    #[test]
    fn blocking_resolver_set_servers_valid() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.set_servers(&["8.8.8.8", "8.8.4.4"]);
        assert!(result.is_ok());
    }

    #[test]
    fn blocking_resolver_set_sortlist_valid() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.set_sortlist(&["130.155.160.0/255.255.240.0"]);
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(cares1_22)]
    fn blocking_resolver_reinit() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.reinit();
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(cares1_24)]
    fn blocking_resolver_get_servers() {
        let resolver = BlockingResolver::new().unwrap();
        let _ = resolver.set_servers(&["8.8.8.8"]);
        let servers = resolver.get_servers();
        assert!(!servers.is_empty());
    }
}
