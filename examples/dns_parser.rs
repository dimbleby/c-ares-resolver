extern crate c_ares_resolver;
extern crate dns_parser;
extern crate tokio_core;

use std::error::Error;
use std::thread;
use std::time::Duration;

use c_ares_resolver::{
    Options,
    Resolver
};
use dns_parser::Packet;

fn main() {
    // Create Resolver.
    let options = Options::new();
    let resolver = Resolver::new(options)
        .expect("Failed to create resolver");

    // Make an A-query, but use a third-party DNS parser to handle the result.
    resolver.query(
        "apple.com",
        1,  // internet
        1,  // Host address
        |result| {
            match result {
                Err(e) => {
                    println!("Query failed with error '{}'", e.description());
                },
                Ok(bytes) => {
                    match Packet::parse(bytes) {
                        Err(e) => {
                            println!(
                                "Parser failed with error '{}'",
                                e.description()
                            );
                        },
                        Ok(packet) => {
                            for answer in &packet.answers {
                                println!("{:?}", answer.data);
                            }
                        },
                    }
                },
            }
        }
    );
    thread::sleep(Duration::from_millis(100));
}
