# Pile


https://user-images.githubusercontent.com/14347895/158874499-bd2c06d6-b922-423b-8134-1bfdf09b8fcc.mov


Note: Pile is currently at POC stage

Pile is a CLI for taking notes that allows you to organize and query your notes by defining rules.

Pile's philosophy is "write first, organize automatically" - creating a new note is as frictionless as possible, and when created it is "thrown in the pile". The note will automatically be matched to existing rules or be updated to match new rules when they are created.

Pile rules are a set of keywords that if present in the note body, the note will be considered to match the rule.
For example: if we have a note with the contents "use `retain()` to remove items from a vector", and a rule "programming" with the keywords "rust", "code", "cli" and "vector", the note above will match the rule. We can then search our notes by specifying the rule name "programming" and be able to query all matching notes.

Pile saves all notes and rules as `.toml` files at `~/.pile`.

## Commands

```sh
pile note <NAME> <CONTENT> - creates a new note
pile rule <NAME> - 
  -k --keyword <KEYWORD> - adds a keyword to the rule
  -r --remove <KEYWORD> - removes a keyword from the rule (not implemented)
  -d --delete - deletes the rule (not implemented)
pile search <QUERY> - searches note names, contents and matching rules
pile index - rematches all notes to rules
pile delete <NAME> - deletes all notes with a specific name
```

## Possible Future Features

- [ ] Regex rules
- [ ] Reminders
- [ ] Automatic remind rules