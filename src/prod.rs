use draft;

pub fn generate_snippets(reverse: bool) -> Vec<String> {
    // todo: figure out why function returns doubly quoted strings (makes file look ugly)
    let mut snippets = vec![];
    for each_tag in draft::generator(reverse).unwrap() {
        let line1 = format!("{} ({})",
                            each_tag.get("version").unwrap(),
                            each_tag.get("date").unwrap());
        let line2 = "-".repeat(line1.len());
        let mut description = vec![];
        for desc in each_tag.get("description").unwrap().as_array().unwrap() {
            let desc_str = desc.to_string();
            if desc_str.starts_with("-") {
                description.push(desc_str)
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
