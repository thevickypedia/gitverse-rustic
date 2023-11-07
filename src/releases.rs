use std::collections::HashMap;

use log::warn;

use git;

fn generate_failed_hash() -> HashMap<String, Vec<String>> {
    let mut hashmap = HashMap::new();
    let vector = Vec::new();
    hashmap.insert("failed".to_string(), vector);
    return hashmap;
}

pub fn get_api_releases() -> HashMap<String, Vec<String>> {
    let origin = git::run(
        r"git config --get remote.origin.url | sed 's/.*\/\([^ ]*\/[^.]*\).*/\1/'"
    );
    if origin.is_empty() {
        warn!("Unable to get origin for current repository");
        return generate_failed_hash();
    }
    if origin == "FAILED".to_string() {
        return generate_failed_hash();
    }
    let origin_info: Vec<&str> = origin.trim().split("/").collect();
    if origin_info.len() != 2 {
        return generate_failed_hash();
    }
    let owner = origin_info[0];
    let repo = origin_info[1];
    println!("OWNER: {}\tREPO: {}", owner, repo);
    return generate_failed_hash();  // todo: be gone
}
