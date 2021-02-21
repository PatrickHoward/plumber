mod cache;
mod config;
mod input;
mod script_builders;

use std::{env, process::Command};

use crate::{config::Config, script_builders::*};

use crate::script_builders::pipefile::PipeFile;
use dirs;

fn show_help() {
    println!("Plumber - Help");
    println!("Usage: plumber {{new|push|build|config|init|help}} {{arguments}}");
    println!("new - Creates a new app or depot");
    println!("push - Attempts to deploy to Steam");
    println!("build - Creates plumber cache where files get stored");
    println!("config - Modify the plumber global configuration");
    println!("help - Prints this message")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let local_directory = env::current_dir().unwrap().clone();
    let local_dir_path = local_directory.to_str().unwrap().to_string();

    if args.len() <= 1 {
        show_help();
        return;
    }

    let config_path = dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.plumber.conf";
    let config = config::Config::get_or_create(&config_path);

    let cache = cache::Cache::locate_repo_cache(local_dir_path.as_str());

    let command = &args[1];

    match command.as_str() {
        "new" => println!("New!"),
        "init" => script_builders::pipefile::PipeFile::new_pipefile(local_dir_path.as_str()),
        "build" => build_cmd(&args[2..args.len() - 1], cache),
        _ => show_help(),
    };
}

pub fn build_cmd(args: &[String], cache: Option<cache::Cache>) {
    if cache.is_none() {}
}
