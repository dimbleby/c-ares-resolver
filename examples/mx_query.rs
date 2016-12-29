// This example demonstrates use of the `FutureResolver`.
extern crate c_ares;
extern crate c_ares_resolver;
extern crate tokio_core;

use std::error::Error;

use c_ares_resolver::FutureResolver;

fn print_mx_results(result: &Result<c_ares::MXResults, c_ares::Error>) {
    match *result {
        Err(ref e) => {
            println!("MX lookup failed with error '{}'", e.description());
        }
        Ok(ref mx_results) => {
            println!("Successful MX lookup...");
            for mx_result in mx_results {
                println!(
                    "host {}, priority {}",
                    mx_result.host(),
                    mx_result.priority()
                );
            }
        }
    }
}

fn main() {
    // Create Resolver and make a query.
    let resolver = FutureResolver::new().expect("Failed to create resolver");
    let query = resolver.query_mx("gmail.com");

    // Run the query to completion and print the results.
    let mut event_loop = tokio_core::reactor::Core::new()
        .expect("Failed to create event loop");
    let result = event_loop.run(query);
    print_mx_results(&result);
}
