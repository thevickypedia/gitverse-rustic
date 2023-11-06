mod parse;

fn main() {
    let (reverse, debug, filename, title) = parse::arguments();
    println!("Reverse: {}", reverse);
    println!("Debug: {}", debug);
    println!("Filename: {}", filename);
    println!("Title: {}", title);
}
