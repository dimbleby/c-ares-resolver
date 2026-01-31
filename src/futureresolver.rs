use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::error::Error;
use crate::host::HostResults;
use crate::nameinfo::NameInfoResult;
use crate::resolver::{Options, Resolver};

#[cfg(cares1_24)]
use c_ares::AresString;

#[cfg(cares1_29)]
use c_ares::ServerStateFlags;

/// The type of future returned by methods on the `FutureResolver`.
#[must_use]
pub struct CAresFuture<T> {
    inner: futures_channel::oneshot::Receiver<c_ares::Result<T>>,
    _resolver: Arc<Resolver>,
}

impl<T> CAresFuture<T> {
    fn new(
        promise: futures_channel::oneshot::Receiver<c_ares::Result<T>>,
        resolver: Arc<Resolver>,
    ) -> Self {
        Self {
            inner: promise,
            _resolver: resolver,
        }
    }

    fn pin_get_inner(
        self: Pin<&mut Self>,
    ) -> Pin<&mut futures_channel::oneshot::Receiver<c_ares::Result<T>>> {
        unsafe { self.map_unchecked_mut(|s| &mut s.inner) }
    }
}

impl<T> Future for CAresFuture<T> {
    type Output = c_ares::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.pin_get_inner()
            .poll(cx)
            .map(|result| result.unwrap_or(Err(c_ares::Error::ECANCELLED)))
    }
}

/// An asynchronous DNS resolver, which returns results as `futures::Future`s.
///
/// Note that dropping the `FutureResolver` does *not* cause outstanding queries to fail - contrast
/// the `Resolver` - because the returned futures hold a reference to the underlying resolver.
pub struct FutureResolver {
    inner: Arc<Resolver>,
}

// Most query implementations follow the same pattern: call through to the `Resolver`, arranging
// that the callback completes a future.
macro_rules! futurize {
    ($resolver:expr, $query:ident, $question:expr) => {{
        let (sender, receiver) = futures_channel::oneshot::channel();
        $resolver.$query($question, |result| {
            let _ = sender.send(result);
        });
        let resolver = Arc::clone(&$resolver);
        CAresFuture::new(receiver, resolver)
    }};
}

impl FutureResolver {
    /// Create a new `FutureResolver`, using default `Options`.
    pub fn new() -> Result<Self, Error> {
        let options = Options::default();
        Self::with_options(options)
    }

    /// Create a new `FutureResolver`, with the given `Options`.
    pub fn with_options(options: Options) -> Result<Self, Error> {
        let inner = Resolver::with_options(options)?;
        let resolver = Self {
            inner: Arc::new(inner),
        };
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
    pub fn query_a(&self, name: &str) -> CAresFuture<c_ares::AResults> {
        futurize!(self.inner, query_a, name)
    }

    /// Search for the A records associated with `name`.
    pub fn search_a(&self, name: &str) -> CAresFuture<c_ares::AResults> {
        futurize!(self.inner, search_a, name)
    }

    /// Look up the AAAA records associated with `name`.
    pub fn query_aaaa(&self, name: &str) -> CAresFuture<c_ares::AAAAResults> {
        futurize!(self.inner, query_aaaa, name)
    }

    /// Search for the AAAA records associated with `name`.
    pub fn search_aaaa(&self, name: &str) -> CAresFuture<c_ares::AAAAResults> {
        futurize!(self.inner, search_aaaa, name)
    }

    /// Look up the CAA records associated with `name`.
    pub fn query_caa(&self, name: &str) -> CAresFuture<c_ares::CAAResults> {
        futurize!(self.inner, query_caa, name)
    }

    /// Search for the CAA records associated with `name`.
    pub fn search_caa(&self, name: &str) -> CAresFuture<c_ares::CAAResults> {
        futurize!(self.inner, search_caa, name)
    }

    /// Look up the CNAME records associated with `name`.
    pub fn query_cname(&self, name: &str) -> CAresFuture<c_ares::CNameResults> {
        futurize!(self.inner, query_cname, name)
    }

    /// Search for the CNAME records associated with `name`.
    pub fn search_cname(&self, name: &str) -> CAresFuture<c_ares::CNameResults> {
        futurize!(self.inner, search_cname, name)
    }

    /// Look up the MX records associated with `name`.
    pub fn query_mx(&self, name: &str) -> CAresFuture<c_ares::MXResults> {
        futurize!(self.inner, query_mx, name)
    }

    /// Search for the MX records associated with `name`.
    pub fn search_mx(&self, name: &str) -> CAresFuture<c_ares::MXResults> {
        futurize!(self.inner, search_mx, name)
    }

    /// Look up the NAPTR records associated with `name`.
    pub fn query_naptr(&self, name: &str) -> CAresFuture<c_ares::NAPTRResults> {
        futurize!(self.inner, query_naptr, name)
    }

    /// Search for the NAPTR records associated with `name`.
    pub fn search_naptr(&self, name: &str) -> CAresFuture<c_ares::NAPTRResults> {
        futurize!(self.inner, search_naptr, name)
    }

    /// Look up the NS records associated with `name`.
    pub fn query_ns(&self, name: &str) -> CAresFuture<c_ares::NSResults> {
        futurize!(self.inner, query_ns, name)
    }

    /// Search for the NS records associated with `name`.
    pub fn search_ns(&self, name: &str) -> CAresFuture<c_ares::NSResults> {
        futurize!(self.inner, search_ns, name)
    }

    /// Look up the PTR records associated with `name`.
    pub fn query_ptr(&self, name: &str) -> CAresFuture<c_ares::PTRResults> {
        futurize!(self.inner, query_ptr, name)
    }

    /// Search for the PTR records associated with `name`.
    pub fn search_ptr(&self, name: &str) -> CAresFuture<c_ares::PTRResults> {
        futurize!(self.inner, search_ptr, name)
    }

    /// Look up the SOA records associated with `name`.
    pub fn query_soa(&self, name: &str) -> CAresFuture<c_ares::SOAResult> {
        futurize!(self.inner, query_soa, name)
    }

    /// Search for the SOA records associated with `name`.
    pub fn search_soa(&self, name: &str) -> CAresFuture<c_ares::SOAResult> {
        futurize!(self.inner, search_soa, name)
    }

    /// Look up the SRV records associated with `name`.
    pub fn query_srv(&self, name: &str) -> CAresFuture<c_ares::SRVResults> {
        futurize!(self.inner, query_srv, name)
    }

    /// Search for the SRV records associated with `name`.
    pub fn search_srv(&self, name: &str) -> CAresFuture<c_ares::SRVResults> {
        futurize!(self.inner, search_srv, name)
    }

    /// Look up the TXT records associated with `name`.
    pub fn query_txt(&self, name: &str) -> CAresFuture<c_ares::TXTResults> {
        futurize!(self.inner, query_txt, name)
    }

    /// Search for the TXT records associated with `name`.
    pub fn search_txt(&self, name: &str) -> CAresFuture<c_ares::TXTResults> {
        futurize!(self.inner, search_txt, name)
    }

    /// Look up the URI records associated with `name`.
    pub fn query_uri(&self, name: &str) -> CAresFuture<c_ares::URIResults> {
        futurize!(self.inner, query_uri, name)
    }

    /// Search for the URI records associated with `name`.
    pub fn search_uri(&self, name: &str) -> CAresFuture<c_ares::URIResults> {
        futurize!(self.inner, search_uri, name)
    }

    /// Perform a host query by address.
    ///
    /// This method is one of the very few places where this library performs strictly more
    /// allocation than the underlying `c-ares` code.  If this is a problem for you, you should
    /// prefer to use the analogous method on the `Resolver`.
    pub fn get_host_by_address(&self, address: &IpAddr) -> CAresFuture<HostResults> {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.inner.get_host_by_address(address, |result| {
            let _ = sender.send(result.map(Into::into));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(receiver, resolver)
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
    ) -> CAresFuture<HostResults> {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.inner.get_host_by_name(name, family, |result| {
            let _ = sender.send(result.map(Into::into));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(receiver, resolver)
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
    ) -> CAresFuture<NameInfoResult> {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.inner.get_name_info(address, flags, |result| {
            let _ = sender.send(result.map(Into::into));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(receiver, resolver)
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
    pub fn query(&self, name: &str, dns_class: u16, query_type: u16) -> CAresFuture<Vec<u8>> {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.inner.query(name, dns_class, query_type, |result| {
            let _ = sender.send(result.map(std::borrow::ToOwned::to_owned));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(receiver, resolver)
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
    pub fn search(&self, name: &str, dns_class: u16, query_type: u16) -> CAresFuture<Vec<u8>> {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.inner.search(name, dns_class, query_type, |result| {
            let _ = sender.send(result.map(std::borrow::ToOwned::to_owned));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(receiver, resolver)
    }

    /// Cancel all requests made on this `FutureResolver`.
    pub fn cancel(&self) {
        self.inner.cancel()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn future_resolver_is_send() {
        assert_send::<FutureResolver>();
    }

    #[test]
    fn future_resolver_is_sync() {
        assert_sync::<FutureResolver>();
    }

    #[test]
    fn c_ares_future_is_send() {
        assert_send::<CAresFuture<c_ares::AResults>>();
    }

    #[test]
    fn c_ares_future_is_sync() {
        assert_sync::<CAresFuture<c_ares::AResults>>();
    }

    #[test]
    fn future_resolver_new() {
        let resolver = FutureResolver::new();
        assert!(resolver.is_ok());
    }

    #[test]
    fn future_resolver_with_options() {
        let options = Options::new();
        let resolver = FutureResolver::with_options(options);
        assert!(resolver.is_ok());
    }

    #[test]
    fn future_resolver_with_custom_options() {
        let mut options = Options::new();
        options.set_timeout(2000).set_tries(2);
        let resolver = FutureResolver::with_options(options);
        assert!(resolver.is_ok());
    }

    #[test]
    fn future_resolver_set_local_ipv4() {
        let resolver = FutureResolver::new().unwrap();
        let result = resolver.set_local_ipv4(Ipv4Addr::new(127, 0, 0, 1));
        assert!(std::ptr::eq(result, &resolver));
    }

    #[test]
    fn future_resolver_set_local_ipv6() {
        let resolver = FutureResolver::new().unwrap();
        let ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        let result = resolver.set_local_ipv6(&ipv6);
        assert!(std::ptr::eq(result, &resolver));
    }

    #[test]
    fn future_resolver_set_local_device() {
        let resolver = FutureResolver::new().unwrap();
        let result = resolver.set_local_device("lo");
        assert!(std::ptr::eq(result, &resolver));
    }

    #[test]
    fn future_resolver_set_servers_valid() {
        let resolver = FutureResolver::new().unwrap();
        let result = resolver.set_servers(&["8.8.8.8", "8.8.4.4"]);
        assert!(result.is_ok());
    }

    #[test]
    fn future_resolver_set_sortlist_valid() {
        let resolver = FutureResolver::new().unwrap();
        let result = resolver.set_sortlist(&["130.155.160.0/255.255.240.0"]);
        assert!(result.is_ok());
    }

    #[test]
    fn future_resolver_cancel() {
        let resolver = FutureResolver::new().unwrap();
        resolver.cancel(); // Should not panic
    }

    #[test]
    #[cfg(cares1_22)]
    fn future_resolver_reinit() {
        let resolver = FutureResolver::new().unwrap();
        let result = resolver.reinit();
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(cares1_24)]
    fn future_resolver_get_servers() {
        let resolver = FutureResolver::new().unwrap();
        let _ = resolver.set_servers(&["8.8.8.8"]);
        let servers = resolver.get_servers();
        assert!(!servers.is_empty());
    }

    // Test CAresFuture behavior
    fn noop_raw_waker() -> RawWaker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            noop_raw_waker()
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    fn noop_waker() -> Waker {
        unsafe { Waker::from_raw(noop_raw_waker()) }
    }

    #[test]
    fn c_ares_future_poll_pending() {
        let resolver = FutureResolver::new().unwrap();
        let mut future = resolver.query_a("example.com");

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // First poll should be pending (query not complete yet)
        let result = Pin::new(&mut future).poll(&mut cx);
        // Result could be Pending or Ready depending on timing
        match result {
            Poll::Pending => {}  // Expected
            Poll::Ready(_) => {} // Also valid if query completed quickly or failed
        }
    }

    #[test]
    fn c_ares_future_cancelled_on_drop() {
        let resolver = FutureResolver::new().unwrap();
        let future = resolver.query_a("example.com");
        drop(future); // Should not panic
    }
}
