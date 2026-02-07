//! Integration tests for the blocking resolver.

mod common;

use c_ares_resolver::BlockingResolver;
use common::test_options;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
