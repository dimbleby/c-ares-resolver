// This example uses the `BlockingResolver` with `send_dnsrec` to perform a DNS
// lookup and inspect the full details of the returned `DnsRecord`: header
// fields, question section, and resource records with type-specific data.
use c_ares::{DnsCls, DnsRecordType, DnsRr, DnsRrKey, DnsSection};
use c_ares_resolver::BlockingResolver;

fn print_rr(rr: &DnsRr) {
    println!(
        "    {} {} {} TTL={}",
        rr.name(),
        rr.rr_type(),
        rr.dns_class(),
        rr.ttl(),
    );
    match rr.rr_type() {
        DnsRecordType::A => {
            if let Some(addr) = rr.get_addr(DnsRrKey::A_ADDR) {
                println!("      Address: {addr}");
            }
        }
        DnsRecordType::AAAA => {
            if let Some(addr) = rr.get_addr6(DnsRrKey::AAAA_ADDR) {
                println!("      Address: {addr}");
            }
        }
        DnsRecordType::CAA => {
            let critical = rr.get_u8(DnsRrKey::CAA_CRITICAL);
            let tag = rr.get_str(DnsRrKey::CAA_TAG).unwrap_or("<none>");
            let value = rr.get_str(DnsRrKey::CAA_VALUE).unwrap_or("<none>");
            println!("      Critical: {critical}, Tag: {tag}, Value: {value}");
        }
        DnsRecordType::CNAME => {
            if let Some(name) = rr.get_str(DnsRrKey::CNAME_CNAME) {
                println!("      CNAME: {name}");
            }
        }
        DnsRecordType::HINFO => {
            let cpu = rr.get_str(DnsRrKey::HINFO_CPU).unwrap_or("<none>");
            let os = rr.get_str(DnsRrKey::HINFO_OS).unwrap_or("<none>");
            println!("      CPU: {cpu}, OS: {os}");
        }
        DnsRecordType::HTTPS => {
            let priority = rr.get_u16(DnsRrKey::HTTPS_PRIORITY);
            let target = rr.get_str(DnsRrKey::HTTPS_TARGET).unwrap_or("<none>");
            println!("      Priority: {priority}, Target: {target}");
            for (i, (key, value)) in rr.opts(DnsRrKey::HTTPS_PARAMS).enumerate() {
                println!("      Param[{i}]: key={key}, value={value:?}");
            }
        }
        DnsRecordType::MX => {
            let pref = rr.get_u16(DnsRrKey::MX_PREFERENCE);
            if let Some(exchange) = rr.get_str(DnsRrKey::MX_EXCHANGE) {
                println!("      Preference: {pref}, Exchange: {exchange}");
            }
        }
        DnsRecordType::NAPTR => {
            let order = rr.get_u16(DnsRrKey::NAPTR_ORDER);
            let pref = rr.get_u16(DnsRrKey::NAPTR_PREFERENCE);
            let flags = rr.get_str(DnsRrKey::NAPTR_FLAGS).unwrap_or("<none>");
            let services = rr.get_str(DnsRrKey::NAPTR_SERVICES).unwrap_or("<none>");
            let regexp = rr.get_str(DnsRrKey::NAPTR_REGEXP).unwrap_or("<none>");
            let replacement = rr.get_str(DnsRrKey::NAPTR_REPLACEMENT).unwrap_or("<none>");
            println!("      Order: {order}, Preference: {pref}");
            println!("      Flags: {flags}, Services: {services}");
            println!("      Regexp: {regexp}, Replacement: {replacement}");
        }
        DnsRecordType::NS => {
            if let Some(ns) = rr.get_str(DnsRrKey::NS_NSDNAME) {
                println!("      Nameserver: {ns}");
            }
        }
        DnsRecordType::OPT => {
            let udp_size = rr.get_u16(DnsRrKey::OPT_UDP_SIZE);
            let version = rr.get_u8(DnsRrKey::OPT_VERSION);
            let flags = rr.get_u16(DnsRrKey::OPT_FLAGS);
            println!("      UDP payload size: {udp_size}");
            println!("      Version: {version}, Flags: {flags:#06x}");
            for (i, (code, data)) in rr.opts(DnsRrKey::OPT_OPTIONS).enumerate() {
                println!("      Option[{i}]: code={code}, data={data:?}");
            }
        }
        DnsRecordType::PTR => {
            if let Some(dname) = rr.get_str(DnsRrKey::PTR_DNAME) {
                println!("      DNAME: {dname}");
            }
        }
        DnsRecordType::SIG => {
            let type_covered = rr.get_u16(DnsRrKey::SIG_TYPE_COVERED);
            let algorithm = rr.get_u8(DnsRrKey::SIG_ALGORITHM);
            let labels = rr.get_u8(DnsRrKey::SIG_LABELS);
            let original_ttl = rr.get_u32(DnsRrKey::SIG_ORIGINAL_TTL);
            let expiration = rr.get_u32(DnsRrKey::SIG_EXPIRATION);
            let inception = rr.get_u32(DnsRrKey::SIG_INCEPTION);
            let key_tag = rr.get_u16(DnsRrKey::SIG_KEY_TAG);
            let signer = rr.get_str(DnsRrKey::SIG_SIGNERS_NAME).unwrap_or("<none>");
            println!("      Type covered: {type_covered}, Algorithm: {algorithm}");
            println!("      Labels: {labels}, Original TTL: {original_ttl}");
            println!("      Expiration: {expiration}, Inception: {inception}");
            println!("      Key tag: {key_tag}, Signer: {signer}");
        }
        DnsRecordType::SOA => {
            let mname = rr.get_str(DnsRrKey::SOA_MNAME).unwrap_or("<none>");
            let rname = rr.get_str(DnsRrKey::SOA_RNAME).unwrap_or("<none>");
            let serial = rr.get_u32(DnsRrKey::SOA_SERIAL);
            let refresh = rr.get_u32(DnsRrKey::SOA_REFRESH);
            let retry = rr.get_u32(DnsRrKey::SOA_RETRY);
            let expire = rr.get_u32(DnsRrKey::SOA_EXPIRE);
            let minimum = rr.get_u32(DnsRrKey::SOA_MINIMUM);
            println!("      MNAME: {mname}");
            println!("      RNAME: {rname}");
            println!("      Serial: {serial}, Refresh: {refresh}, Retry: {retry}");
            println!("      Expire: {expire}, Minimum: {minimum}");
        }
        DnsRecordType::SRV => {
            let priority = rr.get_u16(DnsRrKey::SRV_PRIORITY);
            let weight = rr.get_u16(DnsRrKey::SRV_WEIGHT);
            let port = rr.get_u16(DnsRrKey::SRV_PORT);
            if let Some(target) = rr.get_str(DnsRrKey::SRV_TARGET) {
                println!("      Priority: {priority}, Weight: {weight}, Port: {port}");
                println!("      Target: {target}");
            }
        }
        DnsRecordType::SVCB => {
            let priority = rr.get_u16(DnsRrKey::SVCB_PRIORITY);
            let target = rr.get_str(DnsRrKey::SVCB_TARGET).unwrap_or("<none>");
            println!("      Priority: {priority}, Target: {target}");
            for (i, (key, value)) in rr.opts(DnsRrKey::SVCB_PARAMS).enumerate() {
                println!("      Param[{i}]: key={key}, value={value:?}");
            }
        }
        DnsRecordType::TLSA => {
            let usage = rr.get_u8(DnsRrKey::TLSA_CERT_USAGE);
            let selector = rr.get_u8(DnsRrKey::TLSA_SELECTOR);
            let matching = rr.get_u8(DnsRrKey::TLSA_MATCH);
            println!("      Usage: {usage}, Selector: {selector}, Matching type: {matching}");
            if let Some(data) = rr.get_bin(DnsRrKey::TLSA_DATA) {
                let hex: String = data.iter().map(|b| format!("{b:02x}")).collect();
                println!("      Data: {hex}");
            }
        }
        DnsRecordType::TXT => {
            for (j, data) in rr.abins(DnsRrKey::TXT_DATA).enumerate() {
                let text = String::from_utf8_lossy(data);
                println!("      TXT[{j}]: {text}");
            }
        }
        DnsRecordType::URI => {
            let priority = rr.get_u16(DnsRrKey::URI_PRIORITY);
            let weight = rr.get_u16(DnsRrKey::URI_WEIGHT);
            let target = rr.get_str(DnsRrKey::URI_TARGET).unwrap_or("<none>");
            println!("      Priority: {priority}, Weight: {weight}, Target: {target}");
        }
        DnsRecordType::RAW_RR => {
            let rr_type = rr.get_u16(DnsRrKey::RAW_RR_TYPE);
            println!("      RR type: {rr_type}");
            if let Some(data) = rr.get_bin(DnsRrKey::RAW_RR_DATA) {
                let hex: String = data.iter().map(|b| format!("{b:02x}")).collect();
                println!("      Data: {hex}");
            }
        }
        _ => {
            println!("      (no type-specific display for {:?})", rr.rr_type());
        }
    }
}

fn print_section(record: &c_ares::DnsRecord, section: DnsSection, label: &str) {
    let count = record.rr_count(section);
    if count > 0 {
        println!("  {label} ({count}):");
        for rr in record.rrs(section) {
            print_rr(rr);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    // Initialize winsock.
    let _ = std::net::UdpSocket::bind("127.0.0.1:0");

    let domain = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "google.com".to_string());
    let query_type: DnsRecordType = match std::env::args().nth(2) {
        Some(s) => s
            .parse()
            .map_err(|_| format!("unknown record type '{s}'"))?,
        None => DnsRecordType::A,
    };
    let resolver = BlockingResolver::new()?;

    // Build a DnsRecord query and use send_dnsrec so that we get the full
    // response including Authority and Additional sections, even for error
    // responses like NXDOMAIN.
    let mut query = c_ares::DnsRecord::new(
        0,
        c_ares::DnsFlags::RD,
        c_ares::DnsOpcode::Query,
        c_ares::DnsRcode::NoError,
    )?;
    query.query_add(&domain, query_type, DnsCls::IN)?;

    let record = resolver.send_dnsrec(&query)?;

    println!("Response for {domain} {query_type} (id={}):", record.id());
    println!("  Opcode: {}", record.opcode());
    println!("  Rcode:  {}", record.rcode());
    println!("  Flags:  {:?}", record.flags());

    // Question section.
    let qcount = record.query_count();
    if qcount > 0 {
        println!("  Questions ({qcount}):");
        for (name, qtype, qclass) in record.queries() {
            println!("    {name} {qtype} {qclass}");
        }
    }

    // Resource record sections.
    print_section(&record, DnsSection::Answer, "Answers");
    print_section(&record, DnsSection::Authority, "Authority");
    print_section(&record, DnsSection::Additional, "Additional");

    Ok(())
}
