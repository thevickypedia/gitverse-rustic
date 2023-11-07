use log::warn;

use git;

pub fn generate() -> String {
    let output = git::run("git tag --format '%(refname:short)||%(creatordate:format:%s)'");
    if output.is_empty() {
        warn!("No tags found for repository!!")
    }
    return "".to_string();
}
