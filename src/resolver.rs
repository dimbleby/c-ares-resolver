use std::net::{
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddr,
};
use std::sync::{
    Arc,
    Mutex,
};

use c_ares;

use error::Error;
use eventloop::{
    EventLoop,
    EventLoopHandle
};

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

    /// Set flags controlling the behaviour of the resolver.  The available
    /// flags are documented [here](flags/index.html).
    pub fn set_flags(&mut self, flags: c_ares::flags::Flags) -> &mut Self {
        self.inner.set_flags(flags);
        self
    }

    /// Set the number of milliseconds each name server is given to respond to
    /// a query on the first try.  (After the first try, the timeout algorithm
    /// becomes more complicated, but scales linearly with the value of
    /// timeout).  The default is 5000ms.
    pub fn set_timeout(&mut self, ms: u32) -> &mut Self {
        self.inner.set_timeout(ms);
        self
    }

    /// Set the number of tries the resolver will try contacting each name
    /// server before giving up.  The default is four tries.
    pub fn set_tries(&mut self, tries: u32) -> &mut Self {
        self.inner.set_tries(tries);
        self
    }

    /// Set the number of dots which must be present in a domain name for it to
    /// be queried for "as is" prior to querying for it with the default domain
    /// extensions appended.  The default value is 1 unless set otherwise by
    /// resolv.conf or the RES_OPTIONS environment variable.
    pub fn set_ndots(&mut self, ndots: u32) -> &mut Self {
        self.inner.set_ndots(ndots);
        self
    }

    /// Set the UDP port to use for queries.  The default value is 53, the
    /// standard name service port.
    pub fn set_udp_port(&mut self, udp_port: u16) -> &mut Self {
        self.inner.set_udp_port(udp_port);
        self
    }

    /// Set the TCP port to use for queries.  The default value is 53, the
    /// standard name service port.
    pub fn set_tcp_port(&mut self, tcp_port: u16) -> &mut Self {
        self.inner.set_tcp_port(tcp_port);
        self
    }

    /// Set the domains to search, instead of the domains specified in
    /// resolv.conf or the domain derived from the kernel hostname variable.
    pub fn set_domains(&mut self, domains: &[&str]) -> &mut Self {
        self.inner.set_domains(domains);
        self
    }

    /// Set the lookups to perform for host queries. `lookups` should be set to
    /// a string of the characters "b" or "f", where "b" indicates a DNS lookup
    /// and "f" indicates a lookup in the hosts file.
    pub fn set_lookups(&mut self, lookups: &str) -> &mut Self {
        self.inner.set_lookups(lookups);
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

/// An asynchronous DNS resolver.
pub struct Resolver {
    ares_channel: Arc<Mutex<c_ares::Channel>>,
    event_loop_handle: EventLoopHandle,
}

impl Resolver {
    /// Create a new Resolver.
    pub fn new(options: Options) -> Result<Resolver, Error> {
        // Create and run the event loop.
        let event_loop = try!(EventLoop::new(options.inner));
        let channel = event_loop.ares_channel.clone();
        let handle = event_loop.run();

        // Return the Resolver.
        let resolver = Resolver {
            ares_channel: channel,
            event_loop_handle: handle,
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
        try!(self.ares_channel.lock().unwrap().set_servers(servers));
        Ok(self)
    }

    /// Set the local IPv4 address from which to make queries.
    pub fn set_local_ipv4(&mut self, ipv4: &Ipv4Addr) -> &mut Self {
        self.ares_channel.lock().unwrap().set_local_ipv4(ipv4);
        self
    }

    /// Set the local IPv6 address from which to make queries.
    pub fn set_local_ipv6(&mut self, ipv6: &Ipv6Addr) -> &mut Self {
        self.ares_channel.lock().unwrap().set_local_ipv6(ipv6);
        self
    }

    /// Set the local device from which to make queries.
    pub fn set_local_device(&mut self, device: &str) -> &mut Self {
        self.ares_channel.lock().unwrap().set_local_device(device);
        self
    }

    /// Look up the A records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_a<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::AResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_a(name, handler)
    }

    /// Look up the AAAA records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_aaaa<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::AAAAResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_aaaa(name, handler)
    }

    /// Look up the CNAME records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_cname<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::CNameResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_cname(name, handler)
    }

    /// Look up the MX records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_mx<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::MXResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_mx(name, handler)
    }

    /// Look up the NAPTR records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_naptr<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::NAPTRResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_naptr(name, handler)
    }

    /// Look up the NS records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_ns<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::NSResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_ns(name, handler)
    }

    /// Look up the PTR records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_ptr<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::PTRResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_ptr(name, handler)
    }

    /// Look up the SOA record associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_soa<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::SOAResult, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_soa(name, handler)
    }

    /// Look up the SRV records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_srv<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::SRVResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_srv(name, handler)
    }

    /// Look up the TXT records associated with `name`.
    ///
    /// On completion, `handler` is called with the result.
    pub fn query_txt<F>(&self, name: &str, handler: F) where
        F: FnOnce(Result<c_ares::TXTResults, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap().query_txt(name, handler)
    }

    /// Perform a host query by address.
    ///
    /// On completion, `handler` is called with the result.
    pub fn get_host_by_address<F>(
        &self,
        address: &IpAddr,
        handler: F
    ) where F: FnOnce(Result<c_ares::HostResults, c_ares::Error>) + Send + 'static {
         self.ares_channel.lock().unwrap()
             .get_host_by_address(address, handler)
    }

    /// Perform a host query by name.
    ///
    /// On completion, `handler` is called with the result.
    pub fn get_host_by_name<F>(
        &self,
        name: &str,
        family: c_ares::AddressFamily,
        handler: F
    ) where F: FnOnce(Result<c_ares::HostResults, c_ares::Error>) + Send + 'static {
         self.ares_channel.lock().unwrap()
             .get_host_by_name(name, family, handler);
    }

    /// Address-to-nodename translation in protocol-independent manner.
    ///
    /// On completion, `handler` is called with the result.
    pub fn get_name_info<F>(
        &self,
        address: &SocketAddr,
        flags: c_ares::ni_flags::NIFlags,
        handler: F
    ) where F: FnOnce(Result<c_ares::NameInfoResult, c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap()
            .get_name_info(address, flags, handler)
    }

    /// Perform a DNS query for `name`.  The class and type of the query are
    /// per the provided parameters, taking values as defined in
    /// `arpa/nameser.h`.
    ///
    /// On completion, `handler` is called with the result.
    ///
    /// This method is provided so that users can query DNS types for which
    /// `c-ares` does not provide a parser; or in case a third-party parser is
    /// preferred.  Usually, if a suitable `query_xxx()` is available, that
    /// should be used.
    pub fn query<F>(
        &self,
        name: &str,
        dns_class: u16,
        query_type: u16,
        handler: F
    ) where F: FnOnce(Result<&[u8], c_ares::Error>) + Send + 'static {
        self.ares_channel.lock().unwrap()
            .query(name, dns_class, query_type, handler);
    }

    /// Cancel all requests made on this `Resolver`.
    pub fn cancel(&mut self) {
        self.ares_channel.lock().unwrap().cancel();
    }
}

impl Drop for Resolver {
    fn drop(&mut self) {
        // Shut down the event loop.
        self.event_loop_handle.shutdown();
    }
}
