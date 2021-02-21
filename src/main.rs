mod cache;
mod config;
mod input;
mod script_builders;

use std::{env, process::Command};

use crate::config::Config;

use dirs;

fn show_help() {
    println!("Plumber - Help");
    println!("Usage: plumber {{new|push|config|init|help}} {{arguments}}");
    println!("new - Creates a new app or depot");
    println!("push - Attempts to deploy to Steam");
    println!("config - Modify the plumber global configuration");
    println!("help - Prints this message")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path = dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.plumber.conf";

    if args.len() <= 1 {
        show_help();
        return;
    }

    let config = match config::Config::try_open(config_path.as_str()) {
        Some(c) => c,
        None => config::Config::create_config(config_path.as_str()),
    };

    let command = &args[1];

    match command.as_str() {
        "new" => {}
        _ => show_help(),
    };
}
