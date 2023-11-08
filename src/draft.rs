use std::env;

use log::{error, info, warn};
use serde_json::{Map, Value};
use ::{git, tags};

use parse::Config;
use releases;

pub fn generator(config: Config) -> Option<Vec<Map<String, Value>>> {
    // logger will be enabled only or debug mode
    if config.debug {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    // Don't care about the output since pruning will be successful only if
    // non-existent origin tags are found in local .git
    git::run("git fetch origin refs/tags/*:refs/tags/* --prune");
    let pull = git::run("git pull");
    if pull.is_none() {
        warn!("Failed to git pull");
    }
    // Snippets are generated as Vec<Map<String, Value>> from serde
    // https://stackoverflow.com/a/39147207
    // This allows multiple types of in the same map (dict)
    let tag_notes = tags::get(config.reverse).unwrap();
    if tag_notes.is_empty() {
        error!("Unable to fetch tags");
        return None;
    }
    info!("Git tags gathered: {}", tag_notes.len());
    let release_notes = releases::get();
    let mut updated_tags = Vec::new();
    if !release_notes.is_none() {
        let bind_release_api = release_notes.unwrap();
        if !bind_release_api.is_empty() {
            info!("Release notes gathered: {}", bind_release_api.len());
            for mut tag in tag_notes.clone() {
                let tag_version = tag.get("version").unwrap().as_str().unwrap();
                let api_desc = bind_release_api.get(tag_version);
                // Check if the release version and tag name are the same
                if api_desc.is_some() {
                    let bind_api_desc = api_desc.unwrap();
                    let mut description = vec![];
                    for desc in bind_api_desc {
                        description.push(Value::String(desc.to_string()))
                    }
                    // Update value of key 'tag_version'
                    tag.insert(tag_version.to_string(), Value::Array(description));
                } else {
                    warn!("Tag name: '{}' could not be found in releases", tag_version)
                }
                updated_tags.push(tag)
            }
        }
    }
    return if updated_tags.is_empty() { Some(tag_notes) } else { Some(updated_tags) };
}
