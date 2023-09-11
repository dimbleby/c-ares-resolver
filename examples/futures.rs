// This example demonstrates use of the `FutureResolver`.
extern crate c_ares;
extern crate c_ares_resolver;
extern crate futures_executor;

use c_ares_resolver::FutureResolver;
use futures_executor::block_on;

fn main() {
    #[cfg(windows)]
    // Initialize winsock.
    let _ = std::net::UdpSocket::bind("127.0.0.1:0");

    // Create resolver and make a query.
    let resolver = FutureResolver::new().expect("Failed to create resolver");
    let query = resolver.query_mx("gmail.com");

    // NB Dropping a FutureResolver does *not* cause outstanding requests to fail.
    std::mem::drop(resolver);

    // Run the query to completion.
    let response = block_on(query);

    // Handle the response.
    match response {
        Ok(results) => {
            for result in &results {
                println!(
                    "host {}, priority {}",
                    result.host().to_string_lossy(),
                    result.priority()
                );
            }
        }
        Err(e) => println!("MX lookup failed with error '{}'", e),
    }
}
