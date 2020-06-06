mod commands;
pub mod cratesio;
pub mod utils;
pub mod versions;

use std::env;
use std::io::ErrorKind;

fn handle_error(err: Box<dyn std::error::Error>) {
    if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
        match io_err.kind() {
            ErrorKind::NotFound => println!("ERROR: Cargo.toml file not found"),
            ErrorKind::InvalidData | ErrorKind::InvalidInput => println!("ERROR: {}", err),
            _ => println!("UNEXPECTED ERROR: {}", io_err),
        }
    } else {
        println!("UNEXPECTED ERROR: {}", err);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: 'carp <command>'");
        return;
    }
    match args[1].to_lowercase().as_str() {
        "help" => commands::help(),
        "list" => match commands::list() {
            Ok(_) => (),
            Err(e) => handle_error(e),
        },
        "add" => {
            if args.len() <= 2 {
                println!("'carp add' requires 1 or 2 parameters");
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
                Err(e) => handle_error(e),
            }
        }
        "rem" => {
            if args.len() <= 2 {
                println!("'carp rem' requires 1 parameter");
                return;
            }
            match commands::rem(&args[2]) {
                Ok(message) => println!("{}", message),
                Err(e) => handle_error(e),
            }
        }
        "change" => {
            if args.len() <= 3 {
                println!("'carp change' requires 2 parameters");
                return;
            }
            match commands::change(&args[2], &args[3]) {
                Ok(message) => println!("{}", message),
                Err(e) => handle_error(e),
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
                            println!("All dependencies are up to date");
                        }
                    }
                    Err(e) => handle_error(e),
                }
            } else {
                match commands::check(&args[2]) {
                    Ok(some) => match some {
                        Some(v) => println!("{}", v),
                        None => println!("Dependency '{}' is up to date", args[2]),
                    },
                    Err(e) => handle_error(e),
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
                            println!("All dependencies are up to date");
                        }
                    }
                    Err(e) => handle_error(e),
                }
            } else {
                match commands::update(&args[2]) {
                    Ok(some) => match some {
                        Some(v) => println!("{}", v),
                        None => println!("Dependency '{}' is up to date", args[2]),
                    },
                    Err(e) => handle_error(e),
                }
            }
        }

        unknown => println!(
            "Unknown command '{}'. Use 'carp help' for a list of commands.",
            unknown,
        ),
    }
}
