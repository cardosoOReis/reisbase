use crate::constants::DatabaseStringConstants;
use crate::error_handler::ErrorHandler;
use crate::failures::CustomReisIOFailure;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::{fs, io};

#[derive(Debug, PartialEq, Eq)]
pub struct Reisbase {
    entries: HashMap<String, String>,
}

impl Drop for Reisbase {
    fn drop(&mut self) {
        let contents = self
            .entries
            .iter()
            .map(|(key, value)| format_entry(key, value))
            .collect::<String>();

        _ = fs::write(DatabaseStringConstants::DATABASE_NAME, contents);
    }
}

impl Reisbase {
    pub fn build(db_name: &str) -> Result<Reisbase, CustomReisIOFailure> {
        read_database_contents(db_name)
            .or_else(|err| handle_database_init_failure(err, db_name))
            .map_err(ErrorHandler::handle_io_error)
            .and_then(db_file_to_entries)
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.entries.insert(key.to_string(), value.to_string());
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        self.entries.get(key).map(|value| value.to_owned())
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.entries.remove(key)
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn get_all(&self) -> Option<String> {
        let entries = self
            .entries
            .iter()
            .map(|(key, value)| format_entry(key, value))
            .collect::<String>();

        string_to_option(entries)
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn read_database_contents(name: &str) -> io::Result<String> {
    fs::read_to_string(name)
}

fn create_database_file(db_name: &str) -> io::Result<String> {
    fs::write(db_name, "").map(|_| String::new())
}

fn handle_database_init_failure(error: io::Error, db_name: &str) -> io::Result<String> {
    if let ErrorKind::NotFound = error.kind() {
        create_database_file(db_name)
    } else {
        Err(error)
    }
}

fn db_file_to_entries(contents: String) -> Result<Reisbase, CustomReisIOFailure> {
    let entries_iter = contents
        .lines()
        .filter_map(|line| line.split_once(DatabaseStringConstants::ENTRIES_SEPARATOR))
        .map(|(key, value)| (remove_key_identifier(key), value.to_owned()))
        .collect::<Vec<(String, String)>>();

    let entries = HashMap::from_iter(entries_iter);
    Ok(Reisbase { entries })
}

fn format_entry(key: &str, value: &str) -> String {
    format!(
        "{}{}{}{}\n",
        DatabaseStringConstants::KEY_IDENTIFIER,
        key,
        DatabaseStringConstants::ENTRIES_SEPARATOR,
        value
    )
}

fn remove_key_identifier(key: &str) -> String {
    key.replacen(DatabaseStringConstants::KEY_IDENTIFIER, "", 1)
}

fn string_to_option(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}
