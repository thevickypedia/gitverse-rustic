use draft;

pub fn generate_snippets(reverse: bool) -> Vec<String> {
    let mut snippets = vec![];
    for each_tag in draft::generator(reverse).unwrap() {
        // A rustic approach would be to check if key exists in Map since its returned as an Option
        // if let Some(version) = each_tag.get("version") {
        //     if let Some(version_str) = version.as_str() {

        // A PANIC SAFE approach is to use closure, which will return a None if key doesn't exist
        // let version_str = each_tag.get("version").and_then(|v| v.as_str()).unwrap();

        // However, as values are guaranteed (tags or release notes) I'm using an assumed approach
        let version = each_tag.get("version").unwrap().as_str().unwrap();
        let date = each_tag.get("date").unwrap().as_str().unwrap();
        let line1 = format!("{} ({})", version, date);
        let line2 = "-".repeat(line1.len());
        let mut description: Vec<String> = vec![];
        for desc in each_tag.get("description").unwrap().as_array().unwrap() {
            let desc_str = desc.as_str().unwrap();
            if desc_str.starts_with("-") {
                description.push(desc_str.to_string())
            } else {
                description.push(format!("- {}", desc_str))
            }
        }
        let line3 = description.join("\n");
        let line = format!("{}\n{}\n{}\n", line1, line2, line3);
        snippets.push(line);
    }
    return snippets;
}
