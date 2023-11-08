use chrono::NaiveDateTime;
use log::warn;
use serde_json::{Map, Value};

use git;

pub fn generate() -> Option<Vec<Map<String, Value>>> {
    let dates_values = git::run("git tag --format '%(refname:short)||%(creatordate:format:%s)'");
    if dates_values.is_none() {
        return None;
    }
    let bind_date_values = dates_values.unwrap();
    if bind_date_values.is_empty() {
        warn!("No tags found for repository!!");
        return None;
    }
    let mut snippet: Vec<Map<String, Value>> = Vec::new();
    for line in bind_date_values.split("\n") {
        if line.trim().is_empty() {
            continue;
        }
        let tag_line: Vec<&str> = line.trim().split("||").collect();
        let tag_name = tag_line[0];
        // todo: remove this conversion from here and add it at the end of snippets
        let _timestamp = tag_line[1].parse::<i64>().unwrap_or(0);
        if _timestamp == 0 {
            warn!("Invalid timestamp for tag {}", tag_name);
            continue;
        }
        let timestamp = tag_line[1];
        let datetime = NaiveDateTime::from_timestamp_opt(_timestamp, 0);
        let date = datetime.unwrap().format("%m/%d/%Y").to_string();
        let command = format!("git tag -l -n99 {}", tag_name);
        let notes = git::run(command.as_str());
        if notes.is_none() {
            warn!("No release notes found for tag {}", tag_name);
            continue;
        }
        let bind_notes = notes.unwrap();
        if bind_notes.is_empty() {
            warn!("No release notes found for tag {}", tag_name);
            continue;
        }
        // vector's implementation: https://stackoverflow.com/a/39147207
        let mut vector = vec![];
        for note in bind_notes.trim_start_matches(tag_name).trim().split("\n") {
            vector.push(Value::String(note.to_string()));
        }
        let mut hashmap = Map::new();
        hashmap.insert("version".to_string(), Value::String(tag_name.to_string()));
        hashmap.insert("description".to_string(), Value::Array(vector));
        hashmap.insert("timestamp".to_string(), Value::String(timestamp.to_string()));
        hashmap.insert("date".to_string(), Value::String(date.to_string()));
        snippet.push(hashmap);
    }
    return Some(snippet);
}
