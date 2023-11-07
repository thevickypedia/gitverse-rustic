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

fn build_hashmap(parsed: Vec<JSONObject>) -> HashMap<String, Vec<String>> {
    let mut hashmap = HashMap::new();
    for iter in parsed {
        let mut body = Vec::new();
        for line in iter.body.split("\n") {
            body.push(line.trim().to_string());
        }
        hashmap.insert(iter.name, body);
    }
    return hashmap;
}

fn parse_response(result: reqwest::blocking::Response) -> Option<HashMap<String, Vec<String>>> {
    match result.text() {
        Ok(resp) => {
            match serde_json::from_str(&resp) {
                Ok(parsed) => {
                    return Some(build_hashmap(parsed));
                }
                Err(error_) => {
                    error!("{}", error_);
                }
            }
        }
        Err(error_msg) => {
            error!("{}", error_msg);
        }
    }
    return None;
}

pub fn get_api_releases() -> Option<HashMap<String, Vec<String>>> {
    let origin = git::run(
        r"git config --get remote.origin.url | sed 's/.*\/\([^ ]*\/[^.]*\).*/\1/'"
    );
    if origin.is_empty() {
        warn!("Unable to get origin for current repository");
        return None;
    }
    if origin == "FAILED".to_string() {
        return None;
    }
    let origin_info: Vec<&str> = origin.trim().split("/").collect();
    if origin_info.len() != 2 {
        return None;
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
    match client.build().unwrap().get(&url).send() {
        Ok(result) => {
            if result.status().is_success() {
                return parse_response(result);
            } else {
                warn!("Failed to get releases. {}", result.status())
            }
        }
        Err(error) => {
            error!("{}", error);
        }
    }
    return None;
}
