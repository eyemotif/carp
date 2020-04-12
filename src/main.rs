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
        "check" => {
            if args.len() <= 2 {
                match commands::check_all() {
                    Ok(list) => {
                        let this_list = &list;
                        for line in this_list {
                            println!("{}", line)
                        }
                        if this_list.len() == 0 {
                            println!("All dependencies are up-to-date");
                        }
                    }
                    Err(e) => println!("ERROR: {}", e),
                }
            } else {
                match commands::check(&args[2]) {
                    Ok(some) => match some {
                        Some(v) => println!("{}", v),
                        None => println!("Dependency '{}' is up-to-date", args[2]),
                    },
                    Err(e) => println!("ERROR: {}", e),
                }
            }
        }
        "update" => {
            if args.len() <= 2 {
                match commands::update_all() {
                    Ok(list) => {
                        let this_list = &list;
                        for line in this_list {
                            println!("{}", line)
                        }
                        if this_list.len() == 0 {
                            println!("All dependencies are up-to-date");
                        }
                    }
                    Err(e) => println!("ERROR: {}", e),
                }
            } else {
                match commands::update(&args[2]) {
                    Ok(some) => match some {
                        Some(v) => println!("{}", v),
                        None => println!("Dependency '{}' is up-to-date", args[2]),
                    },
                    Err(e) => println!("ERROR: {}", e),
                }
            }
        }

        unknown => println!(
            "Unknown command '{}'. Use 'carp help' for a list of commands.",
            unknown,
        ),
    }
}
