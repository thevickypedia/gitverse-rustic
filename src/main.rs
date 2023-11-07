extern crate log;

use std::env;

mod parse;
mod git;
mod snippets;

fn main() {
    let config = parse::arguments();
    // todo: remove print statements
    println!("Reverse: {}", config.reverse);
    println!("Debug: {}", config.debug);
    println!("Filename: {}", config.filename);
    println!("Title: {}", config.title);
    // logger will be enabled only or debug mode
    if config.debug {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    let fetch = git::run("git fetch origin refs/tags/*:refs/tags/* --prune");
    if fetch == "FAILED".to_string() {
        log::warn!("Failed to fetch tags");
    }
    let pull = git::run("git pull");
    if pull == "FAILED".to_string() {
        log::warn!("Failed to git pull");
    }
    snippets::generate();
}
