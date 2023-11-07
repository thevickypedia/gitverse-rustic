extern crate log;

use std::env;

use log::{info, warn};

mod parse;
mod git;

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
    if git::run("git fetch origin refs/tags/*:refs/tags/* --prune") &&
        git::run("git pull") {
        info!("Tags updated")
    } else {
        warn!("Failed to refresh tags");
    }
}
