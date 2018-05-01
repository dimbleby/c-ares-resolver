// This example demonstrates use of the `FutureResolver`.
extern crate c_ares;
extern crate c_ares_resolver;
extern crate futures;
extern crate tokio;

use std::error::Error;

use c_ares_resolver::FutureResolver;
use futures::future::Future;

fn main() {
    // Create Resolver and make a query.
    let resolver = FutureResolver::new().expect("Failed to create resolver");
    let query = resolver
        .query_mx("gmail.com")
        .map_err(|e| println!("MX lookup failed with error '{}'", e.description()))
        .map(|results| {
            for result in &results {
                println!("host {}, priority {}", result.host(), result.priority());
            }
        });

    // NB Dropping a FutureResolver does *not* cause outstanding requests to fail.
    std::mem::drop(resolver);

    // Run the query to completion.
    tokio::run(query);
}
