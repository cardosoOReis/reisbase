pub struct DatabaseStringConstants;

impl DatabaseStringConstants {
    pub const DATABASE_NAME: &str = "reis.db";
    pub const KEY_IDENTIFIER: &str = "#-#";
    pub const VALUE_IDENTIFIER: &str = "#$#";
    pub const DESCRIPTION_IDENTIFIER: &str = "#&#";
    pub const ENTRIES_SEPARATOR: &str = "\t";
}

pub struct SucessfulOperationStrings;

impl SucessfulOperationStrings {
    pub fn sucessful_insert_operation(key: &str, value: &str) -> String {
        format!(
            "Sucessfully set the key {} with the value {} in the database!",
            key, value
        )
    }
    pub fn sucessful_delete_operation(key: &str) -> String {
        format!("Sucessfully deleted the entry for {}!", key)
    }
    pub fn sucessful_clear_operation() -> String {
        String::from("Sucessfully cleared all database values!")
    }
}

pub const THIS_ACTION_IS_PERMANENT: &str = "This action is permanent, and will clear all your data. Are you sure you want to continue? (Y/n)";
pub const CANCELED_OPERATION: &str = "The operation was canceled!";
pub const INPUT_READ_ERROR: &str = "Sorry, an error occured when attempting to read your input!";
pub const EMPTY_DATABASE: &str = "Database doesn't contain any value!";

pub fn the_key_already_exists(key: &str, old_value: &str) -> String {
    format!("The key {} already exists in this database, with the value of {}. Do you want to replace it? (Y/n)", key, old_value)
}

pub fn the_entry_does_not_exists(key: &str, value: &str) -> String {
    format!(
        "The entry {} does not exists! You can create a new one with the command: set {} {}",
        key, key, value
    )
}
