use crate::{
    index::index_notes,
    types::{NoteFile, Rule, RuleFile, RuleKind},
};
use std::fs::write;

pub fn matching_rules(contents: &str, rule_file: &RuleFile) -> Vec<String> {
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

pub fn create_rule(
    rule_file: &mut RuleFile,
    note_file: &mut NoteFile,
    rule_name: &str,
    keyword: Option<&str>,
    rules_path: &str,
    notes_path: &str,
) -> anyhow::Result<()> {
    let existing = rule_file
        .rules
        .iter_mut()
        .find(|saved_rule| saved_rule.name == rule_name);

    let keywords = match (keyword, &existing) {
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
                name: rule_name.to_owned(),
                kind: RuleKind::Keywords,
                keywords,
            };
            rule_file.rules.push(new_rule);
        }
    }

    let rules_file_content = toml::to_string(&rule_file)?;
    write(rules_path, rules_file_content)?;

    index_notes(note_file, rule_file, notes_path)?;

    Ok(())
}
