use std::net::{
    Ipv4Addr,
    Ipv6Addr,
};
use c_ares;
use futures;
use futures::Future;

use error::Error;
use resolver::{
    Options,
    Resolver,
};

/// An asynchronous DNS resolver, which returns results as
/// `futures::Future`s.
pub struct FutureResolver {
    inner: Resolver,
}

impl FutureResolver {
    /// Create a new Resolver, using default `Options`.
    pub fn new() -> Result<FutureResolver, Error> {
        let options = Options::default();
        Self::with_options(options)
    }

    /// Create a new Resolver, with the given `Options`.
    pub fn with_options(options: Options) -> Result<FutureResolver, Error> {
        let inner = Resolver::with_options(options)?;
        let resolver = FutureResolver {
            inner: inner,
        };
        Ok(resolver)
    }

    /// Set the list of servers to contact, instead of the servers specified
    /// in resolv.conf or the local named.
    ///
    /// String format is `host[:port]`.  IPv6 addresses with ports require
    /// square brackets eg `[2001:4860:4860::8888]:53`.
    pub fn set_servers(
        &mut self,
        servers: &[&str]) -> Result<&mut Self, c_ares::Error> {
        self.inner.set_servers(servers)?;
        Ok(self)
    }

    /// Set the local IPv4 address from which to make queries.
    pub fn set_local_ipv4(&mut self, ipv4: &Ipv4Addr) -> &mut Self {
        self.inner.set_local_ipv4(ipv4);
        self
    }

    /// Set the local IPv6 address from which to make queries.
    pub fn set_local_ipv6(&mut self, ipv6: &Ipv6Addr) -> &mut Self {
        self.inner.set_local_ipv6(ipv6);
        self
    }

    /// Set the local device from which to make queries.
    pub fn set_local_device(&mut self, device: &str) -> &mut Self {
        self.inner.set_local_device(device);
        self
    }

    /// Look up the A records associated with `name`.
    pub fn query_a(&self, name: &str)
        -> futures::BoxFuture<c_ares::AResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_a(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the A records associated with `name`.
    pub fn search_a(&self, name: &str)
        -> futures::BoxFuture<c_ares::AResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_a(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the AAAA records associated with `name`.
    pub fn query_aaaa(&self, name: &str)
        -> futures::BoxFuture<c_ares::AAAAResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_aaaa(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the AAAA records associated with `name`.
    pub fn search_aaaa(&self, name: &str)
        -> futures::BoxFuture<c_ares::AAAAResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_aaaa(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the CNAME records associated with `name`.
    pub fn query_cname(&self, name: &str)
        -> futures::BoxFuture<c_ares::CNameResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_cname(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the CNAME records associated with `name`.
    pub fn search_cname(&self, name: &str)
        -> futures::BoxFuture<c_ares::CNameResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_cname(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the MX records associated with `name`.
    pub fn query_mx(&self, name: &str)
        -> futures::BoxFuture<c_ares::MXResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_mx(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the MX records associated with `name`.
    pub fn search_mx(&self, name: &str)
        -> futures::BoxFuture<c_ares::MXResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_mx(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the NAPTR records associated with `name`.
    pub fn query_naptr(&self, name: &str)
        -> futures::BoxFuture<c_ares::NAPTRResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_naptr(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the NAPTR records associated with `name`.
    pub fn search_naptr(&self, name: &str)
        -> futures::BoxFuture<c_ares::NAPTRResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_naptr(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the NS records associated with `name`.
    pub fn query_ns(&self, name: &str)
        -> futures::BoxFuture<c_ares::NSResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_ns(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the NS records associated with `name`.
    pub fn search_ns(&self, name: &str)
        -> futures::BoxFuture<c_ares::NSResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_ns(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the PTR records associated with `name`.
    pub fn query_ptr(&self, name: &str)
        -> futures::BoxFuture<c_ares::PTRResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_ptr(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the PTR records associated with `name`.
    pub fn search_ptr(&self, name: &str)
        -> futures::BoxFuture<c_ares::PTRResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_ptr(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the SOA records associated with `name`.
    pub fn query_soa(&self, name: &str)
        -> futures::BoxFuture<c_ares::SOAResult, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_soa(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the SOA records associated with `name`.
    pub fn search_soa(&self, name: &str)
        -> futures::BoxFuture<c_ares::SOAResult, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_soa(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the SRV records associated with `name`.
    pub fn query_srv(&self, name: &str)
        -> futures::BoxFuture<c_ares::SRVResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_srv(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the SRV records associated with `name`.
    pub fn search_srv(&self, name: &str)
        -> futures::BoxFuture<c_ares::SRVResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_srv(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Look up the TXT records associated with `name`.
    pub fn query_txt(&self, name: &str)
        -> futures::BoxFuture<c_ares::TXTResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.query_txt(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Search for the TXT records associated with `name`.
    pub fn search_txt(&self, name: &str)
        -> futures::BoxFuture<c_ares::TXTResults, c_ares::Error> {
        let (c, p) = futures::oneshot();
        self.inner.search_txt(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::Error::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    /// Cancel all requests made on this `FutureResolver`.
    pub fn cancel(&mut self) {
        self.inner.cancel()
    }
}
