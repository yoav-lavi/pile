use crate::{
    rule::matching_rules,
    types::{NoteFile, RuleFile},
};
use std::fs::write;

pub fn index_notes(
    note_file: &mut NoteFile,
    rule_file: &RuleFile,
    notes_path: &str,
) -> anyhow::Result<()> {
    for note in note_file.notes.iter_mut() {
        note.rules = matching_rules(&note.contents, rule_file);
    }
    let note_file_content = toml::to_string(&note_file)?;
    write(notes_path, note_file_content)?;
    Ok(())
}
