extern crate serde;
extern crate serde_json;

use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::input::{query_input_or_default, query_required, query_valid_path};

#[derive(Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Config {
    steamworks_sdk_path: String,
    build_agent_username: String,
    build_agent_password: String,
}

impl Config {
    pub fn try_open(path: &str) -> Option<Self> {
        if let Ok(mut config_file) = std::fs::File::open(path) {
            let mut contents = String::new();
            config_file
                .read_to_string(&mut contents)
                .expect("Failed to open file");

            let config: Config = serde_json::from_str(contents.as_str()).ok()?;

            return Some(config);
        }

        None
    }

    pub fn create_config(path: &str) -> Self {
        let mut output_file =
            std::fs::File::create(path).expect("Failed to create configuration file");
        let steamworks_sdk_path = query_valid_path("Steamworks SDK Location", true);

        println!(
            "Warning!: It's strongly suggested that you do not\n\
        use a personal Steamworks account, consider using a dedicated build account!"
        );

        let build_agent_username = query_required("Build Agent Username");
        let build_agent_password = query_required("Build Agent Password");

        let config = Config {
            steamworks_sdk_path,
            build_agent_username,
            build_agent_password,
        };

        output_file.write(serde_json::to_string(&config).unwrap().as_bytes());

        config
    }
}
