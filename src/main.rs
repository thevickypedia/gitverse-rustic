extern crate chrono;
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;
use std::process::exit;
use std::time::Instant;

use log::debug;

mod parse;
mod git;
mod tags;
mod releases;
mod draft;
mod prod;
mod fileio;

fn main() {
    let start = Instant::now();
    let config = parse::arguments();
    // logger will be enabled only or debug mode
    if config.debug {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    let snippets = prod::generate_snippets(config.reverse);
    if snippets.is_empty() {
        exit(1)
    }
    fileio::dump_release_notes(&config.filename, &config.title, snippets);
    debug!("'{}' was created in: {}s", config.filename, start.elapsed().as_secs());
    exit(0)
}
