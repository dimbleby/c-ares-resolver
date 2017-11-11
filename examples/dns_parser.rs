// This example demonstrates use of the callback-based `Resolver`.  It uses
// a third-party DNS parser to handle the response, rather than `c-ares`'s own
// parser.
extern crate c_ares;
extern crate c_ares_resolver;
extern crate dns_parser;

use std::error::Error;
use std::sync::mpsc;

use c_ares_resolver::Resolver;
use dns_parser::Packet;

fn handle_result(result: &Result<&[u8], c_ares::Error>) {
    match *result {
        Err(ref e) => {
            println!("Query failed with error '{}'", e.description());
        }
        Ok(bytes) => match Packet::parse(bytes) {
            Err(e) => {
                println!("Parser failed with error '{}'", e.description());
            }
            Ok(packet) => for answer in &packet.answers {
                println!("{:?}", answer.data);
            },
        },
    }
}

fn main() {
    // Create Resolver.
    let resolver = Resolver::new().expect("Failed to create resolver");

    // Make an A-query, but use a third-party DNS parser to handle the result.
    let (tx, rx) = mpsc::channel();
    resolver.query(
        "apple.com",
        1, // internet
        1, // Host address
        move |result| {
            handle_result(&result);
            tx.send(()).expect("failed to send on channel!");
        },
    );

    // Don't allow the main thread to exit before the query completes - wait
    // for the handler to signal that it is done.
    rx.recv().expect("query did not complete!");
}
