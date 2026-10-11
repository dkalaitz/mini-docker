mod info;
mod isolation;
mod process;

use std::env;

use info::show_info;
use isolation::{init_child, isolated_child, run_isolated};
use process::run_command;

fn main() {
    println!("MiniDocker starting..");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: minidock <run|info>");
        return;
    }

    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: minidock run <program> [arguments...]");
                return;
            }

            let result = run_command(&args[2], &args[3..]);

            match result {
                Ok(status) => {
                    if status.success() {
                        println!("Process executed");
                    } else {
                        eprintln!("Process didn't exit successfully {}", status);
                    }
                    std::process::exit(status.code().unwrap_or(1));
                }
                Err(err) => {
                    eprintln!("Failed to execute process: {}", err);
                    std::process::exit(1);
                }
            }
        }
        "info" => show_info(),
        "run-isolated" if args.len() >= 3 => match run_isolated(&args[2], &args[3..]) {
            Ok(status) => {
                println!("Exit status: {}", status);
                std::process::exit(status.code().unwrap_or(1));
            }
            Err(error) => {
                eprintln!("Isolation error: {}", error);
                std::process::exit(1);
            }
        },

        "_child" if args.len() >= 3 => {
            if let Err(error) = isolated_child(&args[2], &args[3..]) {
                eprintln!("Child error: {}", error);
                std::process::exit(1);
            }
        }
        "_init" if args.len() >= 3 => {
            if let Err(error) = init_child(&args[2], &args[3..]) {
                eprintln!("Init error: {}", error);
                std::process::exit(1);
            }
        }
        _ => eprintln!("Not a valid command"),
    }
}
