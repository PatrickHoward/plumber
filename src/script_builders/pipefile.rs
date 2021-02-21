use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::{fs, path};

#[derive(Serialize, Deserialize)]
pub struct PipeFile {
    app_id: String,
    depot_ids: Vec<String>,
}

const PIPEFILE_PATH_SUFFIX: &'static str = "/PipeFile";

impl PipeFile {
    pub fn new_pipefile(path_str: &str) {
        let mut pipefile_name = path_str.to_owned();
        pipefile_name.push_str(PIPEFILE_PATH_SUFFIX);

        let path = std::path::Path::new(pipefile_name.as_str());

        if path.exists() {
            eprintln!("PipeFile already exists!");
            return;
        }

        File::create(pipefile_name).unwrap();
        eprintln!("Initialized new PipeFile");
    }

    pub fn get_local_pipefile(path_str: &str) -> Option<PipeFile> {
        let mut pipefile_name = path_str.to_owned();
        pipefile_name.push_str(PIPEFILE_PATH_SUFFIX);

        let path = path::Path::new(pipefile_name.as_str());
        if path.exists() {
            let mut file = File::open(path).expect("Failed to open existing PipeFile");
            let mut contents = String::new();

            file.read_to_string(&mut contents);

            let pipefile: serde_json::Result<PipeFile> = serde_json::from_str(contents.as_str());
            match pipefile {
                Ok(p) => return Some(p),
                Err(_) => return None,
            };
        }

        None
    }
}
