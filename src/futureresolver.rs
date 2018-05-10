use c_ares;
use futures;
use futures::Future;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;

use error::Error;
use host::HostResults;
use nameinfo::NameInfoResult;
use resolver::{Options, Resolver};

/// The type of future returned by methods on the `FutureResolver`.
pub struct CAresFuture<T> {
    inner: futures::sync::oneshot::Receiver<c_ares::Result<T>>,
    _resolver: Arc<Resolver>,
}

impl<T> CAresFuture<T> {
    fn new(
        promise: futures::sync::oneshot::Receiver<c_ares::Result<T>>,
        resolver: Arc<Resolver>,
    ) -> Self {
        CAresFuture {
            inner: promise,
            _resolver: resolver,
        }
    }
}

impl<T> Future for CAresFuture<T> {
    type Item = T;
    type Error = c_ares::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        match self.inner.poll() {
            Ok(futures::Async::NotReady) => Ok(futures::Async::NotReady),
            Err(_) => Err(c_ares::Error::ECANCELLED),
            Ok(futures::Async::Ready(res)) => match res {
                Ok(r) => Ok(futures::Async::Ready(r)),
                Err(e) => Err(e),
            },
        }
    }
}

/// An asynchronous DNS resolver, which returns results as
/// `futures::Future`s.
///
/// Note that dropping the `FutureResolver` does *not* cause outstanding
/// queries to fail - contrast the `Resolver` - because the returned
/// futures hold a reference to the underlying resolver.
pub struct FutureResolver {
    inner: Arc<Resolver>,
}

// Most query implementations follow the same pattern: call through to the
// `Resolver`, arranging that the callback completes a future.
macro_rules! futurize {
    ($resolver:expr, $query:ident, $question:expr) => {{
        let (c, p) = futures::oneshot();
        $resolver.$query($question, move |result| {
            let _ = c.send(result);
        });
        let resolver = Arc::clone(&$resolver);
        CAresFuture::new(p, resolver)
    }};
}

impl FutureResolver {
    /// Create a new `FutureResolver`, using default `Options`.
    pub fn new() -> Result<FutureResolver, Error> {
        let options = Options::default();
        Self::with_options(options)
    }

    /// Create a new `FutureResolver`, with the given `Options`.
    pub fn with_options(options: Options) -> Result<FutureResolver, Error> {
        let inner = Resolver::with_options(options)?;
        let resolver = FutureResolver {
            inner: Arc::new(inner),
        };
        Ok(resolver)
    }

    /// Set the list of servers to contact, instead of the servers specified
    /// in resolv.conf or the local named.
    ///
    /// String format is `host[:port]`.  IPv6 addresses with ports require
    /// square brackets eg `[2001:4860:4860::8888]:53`.
    pub fn set_servers(&self, servers: &[&str]) -> Result<&Self, c_ares::Error> {
        self.inner.set_servers(servers)?;
        Ok(self)
    }

    /// Set the local IPv4 address from which to make queries.
    pub fn set_local_ipv4(&self, ipv4: &Ipv4Addr) -> &Self {
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

    /// Perform a host query by address.
    ///
    /// This method is one of the very few places where this library performs
    /// strictly more allocation than the underlying `c-ares` code.  If this is
    /// a problem for you, you should prefer to use the analogous method on the
    /// `Resolver`.
    pub fn get_host_by_address(&self, address: &IpAddr) -> CAresFuture<HostResults> {
        let (c, p) = futures::oneshot();
        self.inner.get_host_by_address(address, move |result| {
            let _ = c.send(result.map(|h| h.into()));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(p, resolver)
    }

    /// Perform a host query by name.
    ///
    /// This method is one of the very few places where this library performs
    /// strictly more allocation than the underlying `c-ares` code.  If this is
    /// a problem for you, you should prefer to use the analogous method on the
    /// `Resolver`.
    pub fn get_host_by_name(
        &self,
        name: &str,
        family: c_ares::AddressFamily,
    ) -> CAresFuture<HostResults> {
        let (c, p) = futures::oneshot();
        self.inner.get_host_by_name(name, family, move |result| {
            let _ = c.send(result.map(|h| h.into()));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(p, resolver)
    }

    /// Address-to-nodename translation in protocol-independent manner.
    ///
    /// This method is one of the very few places where this library performs
    /// strictly more allocation than the underlying `c-ares` code.  If this is
    /// a problem for you, you should prefer to use the analogous method on the
    /// `Resolver`.
    pub fn get_name_info<F>(
        &self,
        address: &SocketAddr,
        flags: c_ares::NIFlags,
    ) -> CAresFuture<NameInfoResult> {
        let (c, p) = futures::oneshot();
        self.inner.get_name_info(address, flags, move |result| {
            let _ = c.send(result.map(|n| n.into()));
        });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(p, resolver)
    }

    /// Initiate a single-question DNS query for `name`.  The class and type of
    /// the query are per the provided parameters, taking values as defined in
    /// `arpa/nameser.h`.
    ///
    /// This method is one of the very few places where this library performs
    /// strictly more allocation than the underlying `c-ares` code.  If this is
    /// a problem for you, you should prefer to use the analogous method on the
    /// `Resolver`.
    ///
    /// This method is provided so that users can query DNS types for which
    /// `c-ares` does not provide a parser; or in case a third-party parser is
    /// preferred.  Usually, if a suitable `query_xxx()` is available, that
    /// should be used.
    pub fn query(&self, name: &str, dns_class: u16, query_type: u16) -> CAresFuture<Vec<u8>> {
        let (c, p) = futures::oneshot();
        self.inner
            .query(name, dns_class, query_type, move |result| {
                let _ = c.send(result.map(|bs| bs.to_owned()));
            });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(p, resolver)
    }

    /// Initiate a series of single-question DNS queries for `name`.  The
    /// class and type of the query are per the provided parameters, taking
    /// values as defined in `arpa/nameser.h`.
    ///
    /// This method is one of the very few places where this library performs
    /// strictly more allocation than the underlying `c-ares` code.  If this is
    /// a problem for you, you should prefer to use the analogous method on the
    /// `Resolver`.
    ///
    /// This method is provided so that users can search DNS types for which
    /// `c-ares` does not provide a parser; or in case a third-party parser is
    /// preferred.  Usually, if a suitable `search_xxx()` is available, that
    /// should be used.
    pub fn search(&self, name: &str, dns_class: u16, query_type: u16) -> CAresFuture<Vec<u8>> {
        let (c, p) = futures::oneshot();
        self.inner
            .search(name, dns_class, query_type, move |result| {
                let _ = c.send(result.map(|bs| bs.to_owned()));
            });
        let resolver = Arc::clone(&self.inner);
        CAresFuture::new(p, resolver)
    }

    /// Cancel all requests made on this `FutureResolver`.
    pub fn cancel(&self) {
        self.inner.cancel()
    }
}
