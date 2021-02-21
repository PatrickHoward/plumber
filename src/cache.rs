use crate::script_builders::{app_script::AppScript, pipefile::PipeFile};

use std::fs;

pub trait Cacheable {}

pub struct Cache {
    app_script: Option<AppScript>,
    // depot_scripts: Option<Vec<>>,
}

const CACHE_DIRNAME: &'static str = ".plumber";

impl Cache {
    pub fn locate_or_create(path_str: &str, pipefile: &PipeFile) -> Cache {
        match Cache::locate_repo_cache(path_str) {
            Some(cache) => cache,
            None => Cache::create_repo_cache(path_str),
        }
    }

    pub fn locate_repo_cache(path_str: &str) -> Option<Cache> {
        let mut cache_path = path_str.to_owned();
        cache_path.push_str(CACHE_DIRNAME);

        let path = std::path::Path::new(cache_path.as_str());
        if path.is_dir() {
            return Some(Cache { app_script: None });
        }

        None
    }

    pub fn create_repo_cache(path_str: &str) -> Cache {
        let mut cache_path = path_str.to_owned();
        cache_path.push_str(CACHE_DIRNAME);

        fs::create_dir(cache_path.as_str());

        Cache { app_script: None }
    }
}
