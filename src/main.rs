use std::fs::{self, metadata, read_to_string};

use clap::{arg, command, Command};
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Clone)]
struct RuleFile {
    rules: Vec<Rule>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
struct Rule {
    name: String,
    keywords: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct NoteFile {
    notes: Vec<Note>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Note {
    name: String,
    contents: String,
    time: String,
    rules: Vec<String>,
}

#[derive(Error, Debug)]
pub enum PileError {
    #[error("Could not find home directory")]
    CouldNotFindHome,
}

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .subcommand(
            Command::new("note")
                .arg(arg!(<NAME> "Enter a new note"))
                .arg(arg!(<CONTENTS> "The contents of the note")),
        )
        .subcommand(
            Command::new("rule")
                .arg(arg!(<RULE> "Create or edit a rule"))
                .arg(arg!(-k --keyword <KEYWORD> "Add a keyword to a rule").required(false))
                .arg(arg!(-r --remove <KEYWORD> "Remove a keyword from a rule").required(false))
                .arg(arg!(-d --delete "Remove a rule").required(false)),
        )
        .subcommand(Command::new("search").arg(arg!(<QUERY> "Search notes")))
        .subcommand(Command::new("index"))
        .subcommand(Command::new("delete").arg(arg!(<NAME> "Delete a note")))
        .get_matches();

    let home = dirs::home_dir()
        .ok_or(PileError::CouldNotFindHome)?
        .as_path()
        .display()
        .to_string();

    let root = &format!("{}/.pile", home);

    if metadata(root).is_err() {
        fs::create_dir(root)?;
    }

    let rules_path = &format!("{root}/rules.toml");
    let notes_path = &format!("{root}/notes.toml");

    let mut rule_file: RuleFile = if metadata(rules_path).is_ok() {
        toml::from_str(&(read_to_string(rules_path)?))?
    } else {
        RuleFile { rules: vec![] }
    };

    let mut note_file: NoteFile = if metadata(notes_path).is_ok() {
        toml::from_str(&(read_to_string(notes_path)?))?
    } else {
        NoteFile { notes: vec![] }
    };

    if let Some(matches) = matches.subcommand_matches("note") {
        let note_name = matches.value_of("NAME").unwrap();
        let note_contents = matches.value_of("CONTENTS").unwrap();
        let new_note = Note {
            name: note_name.to_owned(),
            contents: note_contents.to_owned(),
            time: OffsetDateTime::now_local()?.to_string(),
            rules: matching_rules(note_contents, &rule_file),
        };

        note_file.notes.push(new_note);

        let note_file_content = toml::to_string(&note_file)?;
        fs::write(notes_path, note_file_content)?
    }

    if let Some(matches) = matches.subcommand_matches("rule") {
        if let Some(rule) = matches.value_of("RULE") {
            let existing = rule_file
                .rules
                .iter_mut()
                .find(|saved_rule| saved_rule.name == rule);

            let keywords = match (matches.value_of("keyword"), &existing) {
                (None, None) => vec![],
                (None, Some(existing)) => existing.keywords.clone(),
                (Some(keyword), None) => vec![keyword.to_owned().to_lowercase()],
                (Some(keyword), Some(existing)) => {
                    let mut new_keywords = existing.keywords.clone();
                    new_keywords.push(keyword.to_owned().to_lowercase());
                    new_keywords
                }
            };

            match existing {
                Some(existing) => {
                    existing.keywords = keywords;
                }
                None => {
                    let new_rule = Rule {
                        name: rule.to_owned(),
                        keywords,
                    };
                    rule_file.rules.push(new_rule);
                }
            }

            let rules_file_content = toml::to_string(&rule_file)?;
            fs::write(rules_path, rules_file_content)?
        }

        index(&mut note_file, &rule_file);

        let note_file_content = toml::to_string(&note_file)?;
        fs::write(notes_path, note_file_content)?;
    }

    if let Some(matches) = matches.subcommand_matches("search") {
        if let Some(query) = matches.value_of("QUERY") {
            note_file.notes.iter().for_each(|note| {
                let rule_string = note.rules.join(" ");

                let search_content =
                    format!("{rule_string} {} {}", note.name, note.contents).to_lowercase();

                if search_content.contains(&query.to_owned().to_lowercase()) {
                    println!(
                        "- {}: {} {}\n",
                        note.name.bright_green(),
                        note.contents.bright_blue(),
                        format!("({})", note.time).dimmed(),
                    );
                }
            })
        }
    }

    if matches.subcommand_matches("index").is_some() {
        index(&mut note_file, &rule_file);
        let note_file_content = toml::to_string(&note_file)?;
        fs::write(notes_path, note_file_content)?;
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(name) = matches.value_of("NAME") {
            note_file.notes.retain(|note| note.name != name);
            let note_file_content = toml::to_string(&note_file)?;
            fs::write(notes_path, note_file_content)?;
        }
    }

    Ok(())
}

fn index(note_file: &mut NoteFile, rule_file: &RuleFile) {
    for note in note_file.notes.iter_mut() {
        note.rules = matching_rules(&note.contents, rule_file);
    }
}

fn matching_rules(contents: &str, rule_file: &RuleFile) -> Vec<String> {
    rule_file
        .rules
        .iter()
        .filter(|rule| {
            rule.keywords
                .iter()
                .any(|keyword| contents.to_lowercase().contains(keyword))
        })
        .map(|rule| rule.name.clone())
        .collect()
}
