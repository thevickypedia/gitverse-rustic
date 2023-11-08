extern crate chrono;
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod parse;
mod git;
mod tags;
mod releases;
mod draft;

fn main() {
    let config = parse::arguments();
    let loaded = draft::generator(config);
    println!("{:?}", loaded)
}
