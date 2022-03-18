use crate::rule::matching_rules;
use crate::types::{Note, NoteFile, RuleFile};
use std::fs::write;
use time::OffsetDateTime;

pub fn create_note(
    note_file: &mut NoteFile,
    rule_file: &RuleFile,
    note_name: &str,
    note_contents: &str,
    notes_path: &str,
) -> anyhow::Result<()> {
    let new_note = Note {
        name: note_name.to_owned(),
        contents: note_contents.to_owned(),
        time: OffsetDateTime::now_local()?.to_string(),
        rules: matching_rules(note_contents, rule_file),
    };

    note_file.notes.push(new_note);

    let note_file_content = toml::to_string(&note_file)?;
    write(notes_path, note_file_content)?;

    Ok(())
}
