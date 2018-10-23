// This example demonstrates use of the callback-based `Resolver`.
extern crate c_ares;
extern crate c_ares_resolver;

use std::error::Error;
use std::sync::mpsc;

use c_ares_resolver::Resolver;

fn print_a_results(result: &c_ares::Result<c_ares::AResults>) {
    match *result {
        Err(ref e) => {
            println!("Query failed with error '{}'", e.description());
        }
        Ok(ref a_results) => {
            println!("Successful A lookup...");
            for a_result in a_results {
                println!("IPv4: {}, TTL {}", a_result.ipv4(), a_result.ttl());
            }
        }
    }
}

fn main() {
    // We'll need to be careful while we're waiting for our callback.  Dropping the resolver would
    // cause the outstanding query to fail - and if we exited the main thread too soon we wouldn't
    // see even that happen.
    //
    // Create a channel that the callback will use to tell the main thread that it is done.
    let (tx, rx) = mpsc::channel();

    // Create a resolver and make a query.
    let resolver = Resolver::new().expect("Failed to create resolver");
    resolver.query_a("apple.com", move |result| {
        print_a_results(&result);
        tx.send(()).unwrap();
    });

    // Wait to be told that the callback has happened.
    rx.recv().unwrap();
}
