//! Integration tests for c-ares-resolver.
//!
//! These tests make actual DNS queries and may be flaky depending on network conditions.
//! Run with `cargo test --test integration` or use `--ignored` to run ignored tests.

use c_ares_resolver::{BlockingResolver, FutureResolver, Options, Resolver};
use futures_executor::block_on;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

// Helper to create a resolver with a short timeout for tests
fn test_options() -> Options {
    let mut options = Options::new();
    options.set_timeout(5000).set_tries(2);
    options
}

mod blocking_resolver {
    use super::*;

    #[test]
    #[ignore = "requires network"]
    fn get_host_by_address() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let addr: IpAddr = "8.8.8.8".parse().unwrap();
        let result = resolver.get_host_by_address(&addr);
        assert!(result.is_ok(), "Failed to get host by address");
        assert!(!result.unwrap().hostname.is_empty());
    }

    #[test]
    #[ignore = "requires network"]
    fn get_host_by_name() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.get_host_by_name("google.com", c_ares::AddressFamily::INET);
        assert!(result.is_ok(), "Failed to get host by name");
        let host = result.unwrap();
        assert!(!host.hostname.is_empty());
    }

    #[test]
    #[ignore = "requires network"]
    fn get_name_info() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
        let result = resolver.get_name_info(&addr, c_ares::NIFlags::empty());
        assert!(result.is_ok(), "Failed to get name info");
        let info = result.unwrap();
        assert!(info.node.is_some() || info.service.is_some());
    }

    #[test]
    #[ignore = "requires network"]
    fn query_a() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_a("google.com");
        assert!(result.is_ok(), "Failed to query A record");
        let records = result.unwrap();
        assert!(
            records.into_iter().next().is_some(),
            "Expected at least one A record"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_aaaa() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_aaaa("google.com");
        assert!(result.is_ok(), "Failed to query AAAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_caa() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_caa("google.com");
        assert!(result.is_ok(), "Failed to query CAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_cname() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_cname("www.github.com");
        assert!(result.is_ok(), "Failed to query CNAME record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_mx() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_mx("google.com");
        assert!(result.is_ok(), "Failed to query MX record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_naptr() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_naptr("sip2sip.info");
        assert!(result.is_ok(), "Failed to query NAPTR record");
    }

    #[test]
    fn query_nonexistent_domain() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_a("this.domain.definitely.does.not.exist.invalid");
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "requires network"]
    fn query_ns() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_ns("google.com");
        assert!(result.is_ok(), "Failed to query NS record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_ptr() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_ptr("8.8.8.8.in-addr.arpa");
        assert!(result.is_ok(), "Failed to query PTR record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_raw() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        // Query type 1 = A record, class 1 = IN
        let result = resolver.query("google.com", 1, 1);
        assert!(result.is_ok(), "Failed raw query");
        let data = result.unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    #[ignore = "requires network"]
    fn query_soa() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_soa("google.com");
        assert!(result.is_ok(), "Failed to query SOA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_srv() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_srv("_imaps._tcp.gmail.com");
        assert!(result.is_ok(), "Failed to query SRV record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_txt() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_txt("google.com");
        assert!(result.is_ok(), "Failed to query TXT record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_uri() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.query_uri("_kerberos.fedoraproject.org");
        assert!(result.is_ok(), "Failed to query URI record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_a() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_a("google.com");
        assert!(result.is_ok(), "Failed to search A record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_aaaa() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_aaaa("google.com");
        assert!(result.is_ok(), "Failed to search AAAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_caa() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_caa("google.com");
        assert!(result.is_ok(), "Failed to search CAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_cname() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_cname("www.github.com");
        assert!(result.is_ok(), "Failed to search CNAME record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_mx() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_mx("google.com");
        assert!(result.is_ok(), "Failed to search MX record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_naptr() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_naptr("sip2sip.info");
        assert!(result.is_ok(), "Failed to search NAPTR record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_ns() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_ns("google.com");
        assert!(result.is_ok(), "Failed to search NS record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_ptr() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_ptr("8.8.8.8.in-addr.arpa");
        assert!(result.is_ok(), "Failed to search PTR record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_raw() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search("google.com", 1, 1);
        assert!(result.is_ok(), "Failed raw search");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_soa() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_soa("google.com");
        assert!(result.is_ok(), "Failed to search SOA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_srv() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_srv("_imaps._tcp.gmail.com");
        assert!(result.is_ok(), "Failed to search SRV record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_txt() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_txt("google.com");
        assert!(result.is_ok(), "Failed to search TXT record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_uri() {
        let resolver = BlockingResolver::with_options(test_options()).unwrap();
        let result = resolver.search_uri("_kerberos.fedoraproject.org");
        assert!(result.is_ok(), "Failed to search URI record");
    }
}

mod future_resolver {
    use super::*;

    #[test]
    #[ignore = "requires network"]
    fn cancel_queries() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_a("example.com");
        resolver.cancel();
        let result = block_on(future);
        assert!(
            result.is_ok() || result.as_ref().err() == Some(&c_ares::Error::ECANCELLED),
            "Expected success or ECANCELLED, got error: {:?}",
            result.err()
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn get_host_by_address() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let addr: IpAddr = "8.8.8.8".parse().unwrap();
        let future = resolver.get_host_by_address(&addr);
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to get host by address");
        assert!(!result.unwrap().hostname.is_empty());
    }

    #[test]
    #[ignore = "requires network"]
    fn get_host_by_name() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.get_host_by_name("google.com", c_ares::AddressFamily::INET);
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to get host by name");
        let host = result.unwrap();
        assert!(!host.hostname.is_empty());
    }

    #[test]
    #[ignore = "requires network"]
    fn get_name_info() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
        let future = resolver.get_name_info(&addr, c_ares::NIFlags::empty());
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to get name info");
        let info = result.unwrap();
        assert!(info.node.is_some() || info.service.is_some());
    }

    #[test]
    #[ignore = "requires network"]
    fn query_a() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_a("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query A record");
        let records = result.unwrap();
        assert!(
            records.into_iter().next().is_some(),
            "Expected at least one A record"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_aaaa() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_aaaa("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query AAAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_caa() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_caa("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query CAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_cname() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_cname("www.github.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query CNAME record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_mx() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_mx("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query MX record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_naptr() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_naptr("sip2sip.info");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query NAPTR record");
    }

    #[test]
    fn query_nonexistent_domain() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_a("this.domain.definitely.does.not.exist.invalid");
        let result = block_on(future);
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "requires network"]
    fn query_ns() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_ns("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query NS record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_ptr() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_ptr("8.8.8.8.in-addr.arpa");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query PTR record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_raw() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query("google.com", 1, 1);
        let result = block_on(future);
        assert!(result.is_ok(), "Failed raw query");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_soa() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_soa("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query SOA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_srv() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_srv("_imaps._tcp.gmail.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query SRV record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_txt() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_txt("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query TXT record");
    }

    #[test]
    #[ignore = "requires network"]
    fn query_uri() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.query_uri("_kerberos.fedoraproject.org");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to query URI record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_a() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_a("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search A record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_aaaa() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_aaaa("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search AAAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_caa() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_caa("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search CAA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_cname() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_cname("www.github.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search CNAME record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_mx() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_mx("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search MX record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_naptr() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_naptr("sip2sip.info");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search NAPTR record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_ns() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_ns("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search NS record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_ptr() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_ptr("8.8.8.8.in-addr.arpa");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search PTR record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_raw() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search("google.com", 1, 1);
        let result = block_on(future);
        assert!(result.is_ok(), "Failed raw search");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_soa() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_soa("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search SOA record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_srv() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_srv("_imaps._tcp.gmail.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search SRV record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_txt() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_txt("google.com");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search TXT record");
    }

    #[test]
    #[ignore = "requires network"]
    fn search_uri() {
        let resolver = FutureResolver::with_options(test_options()).unwrap();
        let future = resolver.search_uri("_kerberos.fedoraproject.org");
        let result = block_on(future);
        assert!(result.is_ok(), "Failed to search URI record");
    }
}

mod callback_resolver {
    use super::*;

    // Helper to wait for a callback with timeout
    fn wait_for_completion(pair: &Arc<(Mutex<bool>, Condvar)>, timeout: Duration) -> bool {
        let (lock, cvar) = pair.as_ref();
        let guard = lock.lock().unwrap();
        let (completed, _) = cvar
            .wait_timeout_while(guard, timeout, |&mut c| !c)
            .unwrap();
        *completed
    }

    #[test]
    #[ignore = "requires network"]
    fn cancel_queries() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_a("example.com", move |result| {
            assert!(
                result.is_ok() || result.as_ref().err() == Some(&c_ares::Error::ECANCELLED),
                "Expected success or ECANCELLED, got error: {:?}",
                result.as_ref().err()
            );
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        resolver.cancel();
        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn get_host_by_address() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);
        let addr: IpAddr = "8.8.8.8".parse().unwrap();

        resolver.get_host_by_address(&addr, move |result| {
            assert!(result.is_ok());
            assert!(!result.unwrap().hostname().is_empty());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn get_host_by_name() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.get_host_by_name("google.com", c_ares::AddressFamily::INET, move |result| {
            assert!(result.is_ok());
            assert!(!result.unwrap().hostname().is_empty());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn get_name_info() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);
        let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();

        resolver.get_name_info(&addr, c_ares::NIFlags::empty(), move |result| {
            assert!(result.is_ok());
            let info = result.unwrap();
            assert!(info.node().is_some() || info.service().is_some());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_a() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_a("google.com", move |result| {
            assert!(result.is_ok());
            assert!(result.unwrap().into_iter().next().is_some());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_aaaa() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_aaaa("google.com", move |result| {
            assert!(result.is_ok());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_mx() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_mx("google.com", move |result| {
            assert!(result.is_ok());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_ns() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_ns("google.com", move |result| {
            assert!(result.is_ok());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_soa() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_soa("google.com", move |result| {
            assert!(result.is_ok());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[ignore = "requires network"]
    fn query_txt() {
        let resolver = Resolver::with_options(test_options()).unwrap();
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_txt("google.com", move |result| {
            assert!(result.is_ok());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Callback was not called"
        );
    }

    #[test]
    #[cfg(cares1_29)]
    #[ignore = "requires network"]
    fn query_with_server_state_callback() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let resolver = Resolver::with_options(test_options()).unwrap();
        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = Arc::clone(&callback_called);

        resolver.set_server_state_callback(move |_server, _success, _flags| {
            callback_called_clone.store(true, Ordering::SeqCst);
        });

        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        resolver.query_a("google.com", move |_result| {
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        });

        assert!(
            wait_for_completion(&pair, Duration::from_secs(3)),
            "Query did not complete"
        );
        assert!(
            callback_called.load(Ordering::SeqCst),
            "Server state callback was not called"
        );
    }
}

mod resolver_configuration {
    use super::*;

    #[test]
    fn set_custom_servers() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.set_servers(&["8.8.8.8", "8.8.4.4"]);
        assert!(result.is_ok());
    }

    #[test]
    fn set_invalid_server() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.set_servers(&["not-a-valid-address"]);
        assert!(result.is_err());
    }

    #[test]
    fn set_ipv6_servers() {
        let resolver = BlockingResolver::new().unwrap();
        let result = resolver.set_servers(&["[2001:4860:4860::8888]:53"]);
        assert!(result.is_ok());
    }

    #[test]
    fn set_local_addresses() {
        let resolver = BlockingResolver::new().unwrap();
        resolver.set_local_ipv4(Ipv4Addr::new(0, 0, 0, 0));
        resolver.set_local_ipv6(&"::".parse().unwrap());
    }
}
