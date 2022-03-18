use colored::Colorize;

use crate::types::NoteFile;

pub fn search_notes(note_file: &NoteFile, query: &str) {
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
