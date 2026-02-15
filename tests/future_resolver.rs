//! Integration tests for the future resolver.

mod common;

use c_ares_resolver::FutureResolver;
use common::test_options;
use futures_executor::block_on;
use std::net::{IpAddr, SocketAddr};

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

#[cfg(cares1_28)]
#[test]
#[ignore = "requires network"]
fn query_dnsrec() {
    use c_ares::{DnsRcode, DnsSection};

    let resolver = FutureResolver::with_options(test_options()).unwrap();
    let future = resolver
        .query_dnsrec("google.com", c_ares::DnsCls::IN, c_ares::DnsRecordType::A)
        .unwrap();
    let result = block_on(future);
    assert!(result.is_ok(), "Failed to query dnsrec");
    let record = result.unwrap();
    assert_eq!(record.rcode(), DnsRcode::NoError);
    assert!(record.rr_count(DnsSection::Answer) > 0);
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
    let resolver = FutureResolver::with_options(test_options()).unwrap();
    let future = resolver.search_dnsrec(&dnsrec).unwrap();
    let result = block_on(future);
    assert!(result.is_ok(), "Failed to search dnsrec");
    let record = result.unwrap();
    assert_eq!(record.rcode(), DnsRcode::NoError);
    assert!(record.rr_count(DnsSection::Answer) > 0);
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
    let resolver = FutureResolver::with_options(test_options()).unwrap();
    let future = resolver.send_dnsrec(&dnsrec).unwrap();
    let result = block_on(future);
    assert!(result.is_ok(), "Failed to send dnsrec");
    let record = result.unwrap();
    assert_eq!(record.rcode(), DnsRcode::NoError);
    assert!(record.rr_count(DnsSection::Answer) > 0);
}
