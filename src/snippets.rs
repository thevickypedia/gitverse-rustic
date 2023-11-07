use std::collections::HashMap;

use chrono::NaiveDateTime;
use log::{error, warn};

use git;

// todo: Currently in a broken state due to borrowed values

pub fn generate() -> Vec<HashMap<& 'static str, & 'static str>> {
    let mut snippet: Vec<HashMap<&str, &str>> = Vec::new();
    let dates_values = git::run("git tag --format '%(refname:short)||%(creatordate:format:%s)'");
    if dates_values.is_empty() {
        warn!("No tags found for repository!!");
        let mut failed = HashMap::new();
        failed.insert("status", "failed");
        snippet.push(failed);
        return snippet;
    }
    if dates_values == "FAILED".to_string() {
        let mut failed = HashMap::new();
        failed.insert("status", "failed");
        snippet.push(failed);
        return snippet;
    }
    for line in dates_values.split("\n") {
        let tag_line: Vec<&str> = line.trim().split("||").collect();
        let tag_name = tag_line[0];
        // todo: remove this conversion from here and add it at the end of snippets
        let _timestamp = tag_line[1].parse::<i64>().unwrap_or(0);
        if _timestamp == 0 {
            let mut failed = HashMap::new();
            failed.insert("status", "failed");
            snippet.push(failed);
            return snippet;
        }
        let timestamp = tag_line[1];
        let datetime = NaiveDateTime::from_timestamp_opt(_timestamp, 0);
        let date = datetime.unwrap().format("%m/%d/%Y").to_string();
        let command = format!("git tag -l -n99 {}", tag_name);
        let notes = git::run(command.as_str());
        if notes.is_empty() {
            warn!("No release notes found for tag {}", tag_name);
            continue;
        }
        if notes == "FAILED".to_string() {
            error!("Failed to get release notes for tag {}", tag_name);
            continue;
        }
        let mut desc = String::new();
        for note in notes.trim_start_matches(tag_name).trim().split("\n") {
            desc.push_str(note)
        }
        let mut hashmap = HashMap::new();
        hashmap.insert("version", tag_name);
        hashmap.insert("description", &desc);
        hashmap.insert("timestamp", timestamp);
        hashmap.insert("date", &date);
        snippet.push(hashmap);
    }
    return snippet;
}
