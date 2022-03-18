mod delete;
mod index;
mod note;
mod rule;
mod search;
mod types;

use clap::{arg, command, Command};
use delete::delete_note;
use index::index_notes;
use note::create_note;
use rule::create_rule;
use search::search_notes;
use std::fs::{self, metadata, read_to_string};
use types::{NoteFile, PileError, RuleFile};

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

    match matches.subcommand() {
        Some(("note", matches)) => {
            let note_name = matches.value_of("NAME").unwrap();
            let note_contents = matches.value_of("CONTENTS").unwrap();
            create_note(
                &mut note_file,
                &rule_file,
                note_name,
                note_contents,
                notes_path,
            )?
        }
        Some(("rule", matches)) => {
            let rule_name = matches.value_of("RULE").unwrap();
            create_rule(
                &mut rule_file,
                &mut note_file,
                rule_name,
                matches.value_of("keyword"),
                rules_path,
                notes_path,
            )?
        }
        Some(("search", matches)) => {
            let query = matches.value_of("QUERY").unwrap();
            search_notes(&note_file, query)
        }
        Some(("index", _)) => index_notes(&mut note_file, &rule_file, notes_path)?,
        Some(("delete", matches)) => {
            let note_name = matches.value_of("NAME").unwrap();
            delete_note(&mut note_file, note_name, notes_path)?
        }
        _ => {}
    }

    Ok(())
}
