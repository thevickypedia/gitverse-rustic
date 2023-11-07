extern crate chrono;
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;

use log::{error, info, warn};

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
    if fetch.is_none() {
        warn!("Failed to fetch tags");
    }
    let pull = git::run("git pull");
    if pull.is_none() {
        warn!("Failed to git pull");
    }
    let tags = snippets::generate().unwrap();
    if tags.is_empty() {
        error!("Unable to fetch tags");
        return;
    }
    info!("Git tags gathered: {}", tags.len());
    let release_api = get_api_releases().unwrap();
    if !release_api.is_empty() {
        info!("Release notes gathered: {}", release_api.len());
    }
}

fn main() {
    let config = parse::arguments();
    generate_release_notes(config)
}
