use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct RuleFile {
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Serialize)]
pub enum RuleKind {
    Keywords,
    Regex,
}

#[derive(Deserialize, Serialize)]
pub struct Rule {
    pub name: String,
    pub kind: RuleKind,
    pub keywords: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct NoteFile {
    pub notes: Vec<Note>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Note {
    pub name: String,
    pub contents: String,
    pub time: String,
    pub rules: Vec<String>,
}

#[derive(Error, Debug)]
pub enum PileError {
    #[error("Could not find home directory")]
    CouldNotFindHome,
}
