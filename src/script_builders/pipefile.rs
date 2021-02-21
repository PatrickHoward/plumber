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
}
