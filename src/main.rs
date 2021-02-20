mod cache;
mod config;
mod input;
mod script_builders;

use std::{env, process::Command};

use crate::config::Config;

fn show_help() {
    println!("Plumber - Help");
    println!("Usage: plumber {{new|push|config|init|help}} {{arguments}}");
    println!("new - Creates a new app or depot");
    println!("push - Attempts to deploy to steam using deploy");
}

const CONFIG_PATH: &'static str = "./plumber.config";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        show_help();
        return;
    }

    let config = match config::Config::try_open(CONFIG_PATH) {
        Some(c) => c,
        None => config::Config::create_config(CONFIG_PATH),
    };

    let command = &args[1];

    match command.as_str() {
        "new" => {}
        _ => show_help(),
    };
}
