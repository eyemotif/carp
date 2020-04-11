mod commands;
pub mod cratesio;
pub mod utils;

use std::env;

fn main() {
    println!("{:?}", cratesio::crate_updated("toml", "0.5.6"));

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: 'carp <command>'");
        return;
    }
    match args[1].to_lowercase().as_str() {
        "help" => commands::help(),
        "list" => commands::list(),
        "add" => {
            if args.len() <= 2 {
                println!("'carp add' requires at least 1 parameter ")
            }
            match commands::add(
                &args[2],
                if args.len() <= 3 {
                    None
                } else {
                    Some(&args[3])
                },
            ) {
                Ok(_) => (),
                Err(e) => println!("ERROR: {}", e),
            }
        }

        unknown => println!(
            "Unknown command '{}'. Use 'carp help' for a list of commands.",
            unknown,
        ),
    }
}
