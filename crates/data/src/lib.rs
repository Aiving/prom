use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    CSharp,
}

#[derive(Default, Serialize, Deserialize)]
pub enum Editor {
    #[default]
    VSCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub languages: Vec<Language>,
    pub last_opened_at: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub editor: Editor,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Request {
    GetProjects,
    AddProject(Project),
    SetEditor(Editor),
    OpenProject(usize),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Response {
    GetProjects(Vec<Project>),
}
