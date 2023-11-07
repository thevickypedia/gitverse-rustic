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
    // todo: move this to a different function
    match client.build().unwrap().get(&url).send() {
        Ok(result) => {
            if result.status().is_success() {
                match result.text() {
                    Ok(resp) => {
                        match serde_json::from_str(&resp) {
                            Ok(parsed) => {
                                let jsonified: Vec<JSONObject> = parsed;
                                let mut hashmap = HashMap::new();
                                for iter in jsonified {
                                    let mut body = Vec::new();
                                    for line in iter.body.split("\n") {
                                        body.push(line.trim().to_string());
                                    }
                                    hashmap.insert(iter.name, body);
                                }
                                return Some(hashmap);
                            }
                            Err(error_) => {
                                error!("{}", error_);
                                return None;
                            }
                        }
                    }
                    Err(error_msg) => {
                        error!("{}", error_msg);
                        return None;
                    }
                }
            } else {
                warn!("Failed to get releases. {}", result.status())
            }
        }
        Err(error) => {
            error!("{}", error);
            return None;
        }
    }
    return None;
}
