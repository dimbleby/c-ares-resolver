use c_ares_resolver::Options;

pub fn test_options() -> Options {
    let mut options = Options::new();
    options.set_timeout(5000).set_tries(2);
    options
}
