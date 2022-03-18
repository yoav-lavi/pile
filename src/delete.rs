use crate::types::NoteFile;
use std::fs::write;

pub fn delete_note(
    note_file: &mut NoteFile,
    note_name: &str,
    notes_path: &str,
) -> anyhow::Result<()> {
    note_file.notes.retain(|note| note.name != note_name);
    let note_file_content = toml::to_string(&note_file)?;
    write(notes_path, note_file_content)?;

    Ok(())
}
