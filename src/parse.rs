use std::env;
use std::process::exit;

pub struct Config {
    pub reverse: bool,
    pub debug: bool,
    pub filename: String,
    pub title: String
}


pub fn arguments() -> Config {
    let args: Vec<String> = env::args().collect();

    let usage = format!(
        "flags:\n\
        \treverse:{}Generate release notes in reverse order\n\
        \tdebug:{}Enable debug mode for detailed logging\n\
        \tversion:{}Get version of the package\n\n\
        arguments:\n\
        \t-f / --filename{}Filename where the release notes should be stored\n\
        \t-t / --title{}Title under which the release notes should be stored",
        " ".repeat(6), " ".repeat(8), " ".repeat(8),
        " ".repeat(7), " ".repeat(10)
    );
    if args.len() < 1 {
        // If no arguments are provided, display usage instructions
        println!("Usage: {} [OPTIONS]\n\n{}", args[0], usage);
        exit(1)
    }

    let mut debug = false;
    let mut reverse = false;

    let mut version = false;
    let mut filename = String::new();
    let mut title = String::new();

    // Loop through the command-line arguments and parse them.
    let mut i = 1; // Start from the second argument (args[0] is the program name).
    while i < args.len() {
        match args[i].as_str() {
            "debug" => {
                debug = true;
            }
            "reverse" => {
                reverse = true;
            }
            "version" => {
                version = true;
            }
            "--filename" | "-f" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    filename = args[i].clone();
                } else {
                    println!("--filename requires a value.");
                    exit(1)
                }
            }
            "--title" | "-t" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    title = args[i].clone();
                } else {
                    println!("--title requires a value.");
                    exit(1)
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                exit(1)
            }
        }
        i += 1;
    }
    if version {
        const PKG_NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("{} {}", PKG_NAME, VERSION);
        exit(0)
    }
    if filename.is_empty() {
        filename = "release_notes.rst".to_string()
    }
    if title.is_empty() {
        title = "Release Notes".to_string()
    }
    let config = Config {
        reverse,
        debug,
        filename,
        title
    };
    return config;
}
