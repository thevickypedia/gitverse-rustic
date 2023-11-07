use std::collections::HashMap;
use std::env;

use log::{debug, error, warn};
use reqwest;

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
    let client = reqwest::blocking::ClientBuilder::new().user_agent("rustc");
    let gh_token = env::var("GIT_TOKEN")
        .unwrap_or(env::var("git_token")
            .unwrap_or("".to_string()));
    let client = if !gh_token.is_empty() {
        debug!("Loading bearer auth with git token");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION,
                       reqwest::header::HeaderValue::from_str(&format!("Bearer {}", gh_token)).unwrap());
        client.default_headers(headers)
    } else {
        warn!("Trying to collect release notes without github token");
        client
    };
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    let request = client.build();
    let response = request.unwrap().get(&url).send();
    match response {
        Ok(ref ok) => {  // borrow the binding pattern todo: add it to notes.md
            let status_code = ok.status().as_u16();
            if status_code == 200 {
                // we need the keys, name and body (split by lines)
                println!("{:?}", response.unwrap().text())
            }
        }
        Err(error) => {
            error!("{}", error);
            return generate_failed_hash();
        }
    }
    return generate_failed_hash();  // todo: be gone
}
