use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub questions: Vec<String>,
    pub actions: Vec<QuestionAction>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Component {
    pub id: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub target: ComponentTarget,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "kind")]
pub enum ComponentTarget {
    Folder { path: String },
    File { path: String },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Question {
    pub prompt: String,
    pub kind: QuestionKind,
    pub subquestions: Vec<Question>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "kind")]
pub enum QuestionKind {
    #[serde(rename = "bool")]
    Bool { default: bool, component: String },
    #[serde(rename = "input")]
    Input { default: String, component: String },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum QuestionAction {
    /// Replace content from files within the given glob pattern.
    #[serde(rename = "replace_content")]
    ReplaceContent { glob: String },
    /// Include the folder from the given path.
    #[serde(rename = "include_folder")]
    IncludeFolder { glob: String },
    /// Rename a folder.
    #[serde(rename = "rename_folder")]
    RenameFolder { glob: String, name: String },
    /// Rename a file.
    #[serde(rename = "rename_file")]
    RenameFile { glob: String, name: String },
}
