mod commands;
pub mod cratesio;
pub mod utils;

use std::env;

fn main() {
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
                println!("'carp add' require 1 or 2 parameters");
                return;
            }
            match commands::add(
                &args[2],
                if args.len() <= 3 {
                    None
                } else {
                    Some(&args[3])
                },
            ) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("ERROR: {}", e),
            }
        }
        "rem" => {
            if args.len() <= 2 {
                println!("'carp rem' requires 1 parameter");
                return;
            }
            match commands::rem(&args[2]) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("ERROR: {}", e),
            }
        }
        "change" => {
            if args.len() <= 3 {
                println!("'carp change' requires 2 parameters")
            }
            match commands::change(&args[2], &args[3]) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("ERROR: {}", e),
            }
        }

        unknown => println!(
            "Unknown command '{}'. Use 'carp help' for a list of commands.",
            unknown,
        ),
    }
}
