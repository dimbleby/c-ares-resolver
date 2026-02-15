//! Integration tests for the callback resolver.

mod common;

use c_ares_resolver::Resolver;
use common::test_options;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

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
fn query_caa() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.query_caa("google.com", move |result| {
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
fn query_cname() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.query_cname("www.github.com", move |result| {
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
fn query_naptr() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.query_naptr("sip2sip.info", move |result| {
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
fn query_nonexistent_domain() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.query_a(
        "this.domain.definitely.does.not.exist.invalid",
        move |result| {
            assert!(result.is_err());
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        },
    );

    assert!(
        wait_for_completion(&pair, Duration::from_secs(3)),
        "Callback was not called"
    );
}

#[test]
#[ignore = "requires network"]
fn query_ptr() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.query_ptr("8.8.8.8.in-addr.arpa", move |result| {
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
fn query_raw() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    // Query type 1 = A record, class 1 = IN
    resolver.query("google.com", 1, 1, move |result| {
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
#[ignore = "requires network"]
fn query_srv() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.query_srv("_imaps._tcp.gmail.com", move |result| {
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
fn query_uri() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.query_uri("_kerberos.fedoraproject.org", move |result| {
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

#[test]
#[ignore = "requires network"]
fn search_a() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_a("google.com", move |result| {
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
fn search_aaaa() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_aaaa("google.com", move |result| {
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
fn search_caa() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_caa("google.com", move |result| {
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
fn search_cname() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_cname("www.github.com", move |result| {
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
fn search_mx() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_mx("google.com", move |result| {
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
fn search_naptr() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_naptr("sip2sip.info", move |result| {
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
fn search_ns() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_ns("google.com", move |result| {
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
fn search_ptr() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_ptr("8.8.8.8.in-addr.arpa", move |result| {
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
fn search_raw() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search("google.com", 1, 1, move |result| {
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
fn search_soa() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_soa("google.com", move |result| {
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
fn search_srv() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_srv("_imaps._tcp.gmail.com", move |result| {
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
fn search_txt() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_txt("google.com", move |result| {
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
fn search_uri() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    resolver.search_uri("_kerberos.fedoraproject.org", move |result| {
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

#[cfg(cares1_28)]
#[test]
#[ignore = "requires network"]
fn query_dnsrec() {
    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let result = resolver.query_dnsrec(
        "google.com",
        c_ares::DnsCls::IN,
        c_ares::DnsRecordType::A,
        move |result| {
            assert!(result.is_ok());
            let record = result.unwrap();
            assert_eq!(record.rcode(), c_ares::DnsRcode::NoError);
            assert!(record.rr_count(c_ares::DnsSection::Answer) > 0);
            let (lock, cvar) = pair_clone.as_ref();
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        },
    );
    assert!(result.is_ok());

    assert!(
        wait_for_completion(&pair, Duration::from_secs(3)),
        "Callback was not called"
    );
}

#[cfg(cares1_28)]
#[test]
#[ignore = "requires network"]
fn search_dnsrec() {
    use c_ares::{DnsCls, DnsFlags, DnsOpcode, DnsRcode, DnsRecord, DnsRecordType, DnsSection};

    let mut dnsrec = DnsRecord::new(0, DnsFlags::RD, DnsOpcode::Query, DnsRcode::NoError).unwrap();
    dnsrec
        .query_add("google.com", DnsRecordType::A, DnsCls::IN)
        .unwrap();

    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let result = resolver.search_dnsrec(&dnsrec, move |result| {
        assert!(result.is_ok());
        let record = result.unwrap();
        assert_eq!(record.rcode(), DnsRcode::NoError);
        assert!(record.rr_count(DnsSection::Answer) > 0);
        let (lock, cvar) = pair_clone.as_ref();
        *lock.lock().unwrap() = true;
        cvar.notify_one();
    });
    assert!(result.is_ok());

    assert!(
        wait_for_completion(&pair, Duration::from_secs(3)),
        "Callback was not called"
    );
}

#[cfg(cares1_28)]
#[test]
#[ignore = "requires network"]
fn send_dnsrec() {
    use c_ares::{DnsCls, DnsFlags, DnsOpcode, DnsRcode, DnsRecord, DnsRecordType, DnsSection};

    let mut dnsrec = DnsRecord::new(0, DnsFlags::RD, DnsOpcode::Query, DnsRcode::NoError).unwrap();
    dnsrec
        .query_add("google.com", DnsRecordType::A, DnsCls::IN)
        .unwrap();

    let resolver = Resolver::with_options(test_options()).unwrap();
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let result = resolver.send_dnsrec(&dnsrec, move |result| {
        assert!(result.is_ok());
        let record = result.unwrap();
        assert_eq!(record.rcode(), DnsRcode::NoError);
        assert!(record.rr_count(DnsSection::Answer) > 0);
        let (lock, cvar) = pair_clone.as_ref();
        *lock.lock().unwrap() = true;
        cvar.notify_one();
    });
    assert!(result.is_ok());

    assert!(
        wait_for_completion(&pair, Duration::from_secs(3)),
        "Callback was not called"
    );
}
