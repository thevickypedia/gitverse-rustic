use std::collections::HashMap;
use std::env;

use log::{debug, error, warn};
use reqwest;
use serde::Deserialize;

use git;

#[derive(Deserialize)]
struct JSONObject {
    name: String,
    body: String,
}

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
    let mut client = reqwest::blocking::ClientBuilder::new().user_agent("rustc");
    let gh_token = env::var("GIT_TOKEN")
        .unwrap_or(env::var("git_token")
            .unwrap_or("".to_string()));
    if !gh_token.is_empty() {
        debug!("Loading bearer auth with git token");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION,
                       reqwest::header::HeaderValue::from_str(&format!("Bearer {}", gh_token)).unwrap());
        client = client.default_headers(headers)
    } else {
        warn!("Trying to collect release notes without github token");
    }
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    let response = client.build().unwrap().get(&url).send();
    match response {
        Ok(result) => {
            let status = result.status();
            if status.is_success() {
                // todo: replace with match and return failed hash
                let response_txt = result.text().expect("Failed to get response as text");
                let parsed_response: Vec<JSONObject> = serde_json::from_str(&response_txt).expect("Failed to parse JSON");
                let mut hashmap = HashMap::new();
                for iter in parsed_response {
                    let mut body = Vec::new();
                    for line in iter.body.split("\n") {
                        body.push(line.trim().to_string());
                    }
                    hashmap.insert(iter.name, body);
                }
                return hashmap;
            } else {
                warn!("Failed to get releases. {}", status)
            }
        }
        Err(error) => {
            error!("{}", error);
            return generate_failed_hash();
        }
    }
    return generate_failed_hash();  // todo: be gone
}
