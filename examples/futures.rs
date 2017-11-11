// This example demonstrates use of the `FutureResolver`.
extern crate c_ares;
extern crate c_ares_resolver;
extern crate futures;
extern crate tokio_core;

use std::error::Error;

use c_ares_resolver::FutureResolver;
use futures::future::Future;

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
    let query = {
        let resolver = FutureResolver::new().expect("Failed to create resolver");
        resolver.query_mx("gmail.com").then(|result| {
            print_mx_results(&result);
            result
        })
    };

    // Run the query to completion and print the results.
    let mut event_loop = tokio_core::reactor::Core::new().expect("Failed to create event loop");
    event_loop.run(query).ok();
}
