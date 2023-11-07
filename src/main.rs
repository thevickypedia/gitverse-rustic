extern crate chrono;
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;

use log::{info, warn};

use parse::Config;
use releases::get_api_releases;

mod parse;
mod git;
mod snippets;
mod releases;

fn generate_release_notes(config: Config) {
    // logger will be enabled only or debug mode
    if config.debug {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    let fetch = git::run("git fetch origin refs/tags/*:refs/tags/* --prune");
    if fetch == "FAILED".to_string() {
        warn!("Failed to fetch tags");
    }
    let pull = git::run("git pull");
    if pull == "FAILED".to_string() {
        warn!("Failed to git pull");
    }
    let generated = snippets::generate();
    if generated[0].get("failed").is_some() {
        return;
    }
    info!("Git tags gathered: {}", generated.len());
    let release_api = get_api_releases();
    if release_api.get("failed").is_some() {
        return;
    }
    info!("Release notes gathered: {}", release_api.len());
    println!("{:?}", release_api)
    // for release in release_api {
    //     println!("{:?}", release)
    // }
}

fn main() {
    let config = parse::arguments();
    generate_release_notes(config)
}
