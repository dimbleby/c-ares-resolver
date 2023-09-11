// This example demonstrates use of the `BlockingResolver`.
extern crate c_ares;
extern crate c_ares_resolver;

use std::str;

use c_ares_resolver::BlockingResolver;

fn print_txt_results(result: &c_ares::Result<c_ares::TXTResults>) {
    match *result {
        Err(ref e) => {
            println!("TXT lookup failed with error '{}'", e);
        }
        Ok(ref txt_results) => {
            println!("Successful TXT lookup...");
            for txt_result in txt_results {
                let text = str::from_utf8(txt_result.text()).unwrap_or("<binary>");
                println!(
                    "record start: {}, text: {}",
                    txt_result.record_start(),
                    text
                );
            }
        }
    }
}

fn main() {
    #[cfg(windows)]
    // Initialize winsock.
    let _ = std::net::UdpSocket::bind("127.0.0.1:0");

    // Create Resolver and make a query.
    let resolver = BlockingResolver::new().expect("Failed to create resolver");
    let result = resolver.query_txt("gmail.com");
    print_txt_results(&result);
}
