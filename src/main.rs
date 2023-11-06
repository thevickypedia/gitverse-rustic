mod parse;

fn main() {
    let config = parse::arguments();
    println!("Reverse: {}", config.reverse);
    println!("Debug: {}", config.debug);
    println!("Filename: {}", config.filename);
    println!("Title: {}", config.title);
}
