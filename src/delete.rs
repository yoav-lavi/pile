use std::fs;

use crate::types::NoteFile;

pub fn delete_note(
    note_file: &mut NoteFile,
    note_name: &str,
    notes_path: &str,
) -> anyhow::Result<()> {
    note_file.notes.retain(|note| note.name != note_name);
    let note_file_content = toml::to_string(&note_file)?;
    fs::write(notes_path, note_file_content)?;

    Ok(())
}
