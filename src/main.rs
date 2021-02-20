pub mod cache;

use std::{env, process::Command};

fn show_help() {
    println!("Plumber - Help");
    println!("Usage: plumber {{new|push|config}} {{arguments}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        match arg.as_str() {
            "new" => new::run(&args),
            "push" => {}
            "config" => {}
            _ => {
                show_help();
                break;
            }
        }
    }
}

mod new {
    fn show_help() {
        println!("Usage: plumber new {{app|depot}} {{name}}");
    }

    pub fn run(args: &Vec<String>) {
        if args.len() < 2 {
            show_help();
            return;
        }
    }
}

fn push_cmd() {
    pub fn run(args: &Vec<String>) {}
}

fn config_cmd() {}
