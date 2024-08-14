use std::{fs, io};

use prom_data::{Config, Project};

use crate::util::{try_read_file, ROOT};

pub struct App {
    pub projects: Vec<Project>,
    pub config: Config,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        if !ROOT.exists() {
            fs::create_dir_all(ROOT.as_path())
                .unwrap_or_else(|_| panic!("failed to create directory at {}", ROOT.display()));
        }

        let projects = try_read_file(ROOT.join("projects.json")).expect("failed to read file");
        let config = try_read_file(ROOT.join("config.json")).expect("failed to read file");

        Self { projects, config }
    }

    pub fn save_projects(&self) -> io::Result<()> {
        fs::write(
            ROOT.join("projects.json"),
            serde_json::to_vec(&self.projects).expect("failed to serialize projects"),
        )
    }

    pub fn save_config(&self) -> io::Result<()> {
        fs::write(
            ROOT.join("config.json"),
            serde_json::to_vec(&self.config).expect("failed to serialize config"),
        )
    }
}
