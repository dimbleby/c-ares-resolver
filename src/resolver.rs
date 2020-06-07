use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::error::Error;
use crate::eventloop::EventLoop;

/// Used to configure the behaviour of the resolver.
#[derive(Default)]
pub struct Options {
    inner: c_ares::Options,
}

impl Options {
    /// Returns a fresh `Options`, on which no values are set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set flags controlling the behaviour of the resolver.
    pub fn set_flags(&mut self, flags: c_ares::Flags) -> &mut Self {
        self.inner.set_flags(flags);
        self
    }

    /// Set the number of milliseconds each name server is given to respond to a query on the first
    /// try.  (After the first try, the timeout algorithm becomes more complicated, but scales
    /// linearly with the value of timeout).  The default is 5000ms.
    pub fn set_timeout(&mut self, ms: u32) -> &mut Self {
        self.inner.set_timeout(ms);
        self
    }

    /// Set the number of tries the resolver will try contacting each name server before giving up.
    /// The default is four tries.
    pub fn set_tries(&mut self, tries: u32) -> &mut Self {
        self.inner.set_tries(tries);
        self
    }

    /// Set the number of dots which must be present in a domain name for it to be queried for "as
    /// is" prior to querying for it with the default domain extensions appended.  The default
    /// value is 1 unless set otherwise by resolv.conf or the RES_OPTIONS environment variable.
    pub fn set_ndots(&mut self, ndots: u32) -> &mut Self {
        self.inner.set_ndots(ndots);
        self
    }

    /// Set the UDP port to use for queries.  The default value is 53, the standard name service
    /// port.
    pub fn set_udp_port(&mut self, udp_port: u16) -> &mut Self {
        self.inner.set_udp_port(udp_port);
        self
    }

    /// Set the TCP port to use for queries.  The default value is 53, the standard name service
    /// port.
    pub fn set_tcp_port(&mut self, tcp_port: u16) -> &mut Self {
        self.inner.set_tcp_port(tcp_port);
        self
    }

    /// Set the domains to search, instead of the domains specified in resolv.conf or the domain
    /// derived from the kernel hostname variable.
    pub fn set_domains(&mut self, domains: &[&str]) -> &mut Self {
        self.inner.set_domains(domains);
        self
    }

    /// Set the lookups to perform for host queries. `lookups` should be set to a string of the
    /// characters "b" or "f", where "b" indicates a DNS lookup and "f" indicates a lookup in the
    /// hosts file.
    pub fn set_lookups(&mut self, lookups: &str) -> &mut Self {
        self.inner.set_lookups(lookups);
        self
    }

    /// The path to use for reading the resolv.conf file.  The `resolvconf_path` should be set to a
    /// path string, and will be honoured on *nix like systems.  The default is /etc/resolv.conf.
    pub fn set_resolvconf_path(&mut self, resolvconf_path: &str) -> &mut Self {
        self.inner.set_resolvconf_path(resolvconf_path);
        self
    }

    /// Set the socket send buffer size.
    pub fn set_sock_send_buffer_size(&mut self, size: u32) -> &mut Self {
        self.inner.set_sock_send_buffer_size(size);
        self
    }

    /// Set the socket receive buffer size.
    pub fn set_sock_receive_buffer_size(&mut self, size: u32) -> &mut Self {
        self.inner.set_sock_receive_buffer_size(size);
        self
    }

    /// Configure round robin selection of nameservers.
    pub fn set_rotate(&mut self) -> &mut Self {
        self.inner.set_rotate();
        self
    }

    /// Prevent round robin selection of nameservers.
    pub fn set_no_rotate(&mut self) -> &mut Self {
        self.inner.set_no_rotate();
        self
    }

    /// Set the EDNS packet size.
    pub fn set_ednspsz(&mut self, size: u32) -> &mut Self {
        self.inner.set_ednspsz(size);
        self
    }
}

/// An asynchronous DNS resolver, which returns results via callbacks.
///
/// Note that dropping the resolver will cause all outstanding requests to fail with result
/// `c_ares::Error::EDESTRUCTION`.
pub struct Resolver {
    ares_channel: Arc<Mutex<c_ares::Channel>>,

    // Set this true to stop the underlying event loop.
    event_loop_stopper: Arc<AtomicBool>,
}

impl Drop for Resolver {
    fn drop(&mut self) {
        self.event_loop_stopper.store(true, Ordering::Relaxed);
    }
}

impl Resolver {
    /// Create a new `Resolver`, using default `Options`.
    pub fn new() -> Result<Self, Error> {
        let options = Options::default();
        Self::with_options(options)
    }

    /// Create a new `Resolver`, with the given `Options`.
    pub fn with_options(options: Options) -> Result<Resolver, Error> {
        // Create and run the event loop.
        let event_loop = EventLoop::new(options.inner)?;
        let channel = Arc::clone(&event_loop.ares_channel);
        let stopper = event_loop.run();

        // Return the Resolver.
        let resolver = Resolver {
            ares_channel: channel,
            event_loop_stopper: stopper,
        };
        Ok(resolver)
    }

    /// Set the list of servers to contact, instead of the servers specified in resolv.conf or the
    /// local named.
    ///
    /// String format is `host[:port]`.  IPv6 addresses with ports require square brackets eg
    /// `[2001:4860:4860::8888]:53`.
    pub fn set_servers(&self, servers: &[&str]) -> c_ares::Result<&Self> {
        self.ares_channel.lock().unwrap().set_servers(servers)?;
        Ok(self)
    }

    /// Set the local IPv4 address from which to make queries.
    pub fn set_local_ipv4(&self, ipv4: Ipv4Addr) -> &Self {
        self.ares_channel.lock().unwrap().set_local_ipv4(ipv4);
        self
    }

    /// Set the local IPv6 address from which to make queries.
    pub fn set_local_ipv6(&self, ipv6: &Ipv6Addr) -> &Self {
        self.ares_channel.lock().unwrap().set_local_ipv6(ipv6);
        self
    }

    /// Set the local device from which to make queries.
    pub fn set_local_device(&self, device: &str) -> &Self {
        self.ares_channel.lock().unwrap().set_local_device(device);
        self
    }

    /// Look up the A records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_a<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::AResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_a(name, handler)
    }

    /// Search for the A records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_a<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::AResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_a(name, handler)
    }

    /// Look up the AAAA records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_aaaa<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::AAAAResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_aaaa(name, handler)
    }

    /// Search for the AAAA records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_aaaa<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::AAAAResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_aaaa(name, handler)
    }

    /// Look up the CNAME records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_cname<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::CNameResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_cname(name, handler)
    }

    /// Search for the CNAME records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_cname<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::CNameResults>) + Send + 'static,
    {
        self.ares_channel
            .lock()
            .unwrap()
            .search_cname(name, handler)
    }

    /// Look up the MX records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_mx<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::MXResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_mx(name, handler)
    }

    /// Search for the MX records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_mx<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::MXResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_mx(name, handler)
    }

    /// Look up the NAPTR records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_naptr<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::NAPTRResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_naptr(name, handler)
    }

    /// Search for the NAPTR records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_naptr<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::NAPTRResults>) + Send + 'static,
    {
        self.ares_channel
            .lock()
            .unwrap()
            .search_naptr(name, handler)
    }

    /// Look up the NS records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_ns<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::NSResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_ns(name, handler)
    }

    /// Search for the NS records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_ns<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::NSResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_ns(name, handler)
    }

    /// Look up the PTR records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_ptr<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::PTRResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_ptr(name, handler)
    }

    /// Search for the PTR records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_ptr<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::PTRResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_ptr(name, handler)
    }

    /// Look up the SOA record associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_soa<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::SOAResult>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_soa(name, handler)
    }

    /// Search for the SOA record associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_soa<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::SOAResult>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_soa(name, handler)
    }

    /// Look up the SRV records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_srv<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::SRVResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_srv(name, handler)
    }

    /// Search for the SRV records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_srv<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::SRVResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_srv(name, handler)
    }

    /// Look up the TXT records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_txt<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::TXTResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().query_txt(name, handler)
    }

    /// Search for the TXT records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn search_txt<F>(&self, name: &str, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::TXTResults>) + Send + 'static,
    {
        self.ares_channel.lock().unwrap().search_txt(name, handler)
    }

    /// Perform a host query by address.
    ///
    /// On completion, `handler` is called with the result.
    pub fn get_host_by_address<F>(&self, address: &IpAddr, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::HostResults>) + Send + 'static,
    {
        self.ares_channel
            .lock()
            .unwrap()
            .get_host_by_address(address, handler)
    }

    /// Perform a host query by name.
    ///
    /// On completion, `handler` is called with the result.
    pub fn get_host_by_name<F>(&self, name: &str, family: c_ares::AddressFamily, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::HostResults>) + Send + 'static,
    {
        self.ares_channel
            .lock()
            .unwrap()
            .get_host_by_name(name, family, handler);
    }

    /// Address-to-nodename translation in protocol-independent manner.
    ///
    /// On completion, `handler` is called with the result.
    pub fn get_name_info<F>(&self, address: &SocketAddr, flags: c_ares::NIFlags, handler: F)
    where
        F: FnOnce(c_ares::Result<c_ares::NameInfoResult>) + Send + 'static,
    {
        self.ares_channel
            .lock()
            .unwrap()
            .get_name_info(address, flags, handler)
    }

    /// Initiate a single-question DNS query for `name`.  The class and type of the query are per
    /// the provided parameters, taking values as defined in `arpa/nameser.h`.
    ///
    /// On completion, `handler` is called with the result.
    ///
    /// This method is provided so that users can query DNS types for which `c-ares` does not
    /// provide a parser; or in case a third-party parser is preferred.  Usually, if a suitable
    /// `query_xxx()` is available, that should be used.
    pub fn query<F>(&self, name: &str, dns_class: u16, query_type: u16, handler: F)
    where
        F: FnOnce(c_ares::Result<&[u8]>) + Send + 'static,
    {
        self.ares_channel
            .lock()
            .unwrap()
            .query(name, dns_class, query_type, handler);
    }

    /// Initiate a series of single-question DNS queries for `name`.  The class and type of the
    /// query are per the provided parameters, taking values as defined in `arpa/nameser.h`.
    ///
    /// On completion, `handler` is called with the result.
    ///
    /// This method is provided so that users can search DNS types for which `c-ares` does not
    /// provide a parser; or in case a third-party parser is preferred.  Usually, if a suitable
    /// `search_xxx()` is available, that should be used.
    pub fn search<F>(&self, name: &str, dns_class: u16, query_type: u16, handler: F)
    where
        F: FnOnce(c_ares::Result<&[u8]>) + Send + 'static,
    {
        self.ares_channel
            .lock()
            .unwrap()
            .search(name, dns_class, query_type, handler);
    }

    /// Cancel all requests made on this `Resolver`.
    pub fn cancel(&self) {
        self.ares_channel.lock().unwrap().cancel();
    }
}
