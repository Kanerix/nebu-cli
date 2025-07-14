use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    questions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Question {
    prompt: String,
    kind: QuestionKind,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QuestionKind {
    #[serde(rename = "boolean")]
    Bool(bool),
    #[serde(rename = "input")]
    Input(String)
}

impl Question {
    pub fn new(prompt: String, kind: QuestionKind) -> Self {
        Question {
            prompt,
            kind,
        }
    }
}
