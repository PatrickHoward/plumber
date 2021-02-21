use serde::{Deserialize, Serialize};

use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct PipeFile {
    app_id: String,
    depot_ids: Vec<String>,
}

const PIPEFILE_PATH_SUFFIX: &'static str = "/PipeFile";

impl PipeFile {
    pub fn new_pipefile(path_str: &str) {
        let path = std::path::Path::new(stringify!("{}{}", path_str, PIPEFILE_PATH_SUFFIX));

        if path.exists() {
            eprintln!("PipeFile already exists!");
            return;
        }

        File::create(stringify!("{}{}", path_str, PIPEFULE_PATH_SUFFIX)).unwrap();
        eprintln!("Initialized new PipeFile");
    }
}
