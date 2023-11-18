pub struct DatabaseStringConstants;

impl DatabaseStringConstants {
    pub const DATABASE_NAME: &str = "reis.db";
    pub const KEY_IDENTIFIER: &str = "#-#";
    pub const VALUE_IDENTIFIER: &str = "#$#";
    pub const DESCRIPTION_IDENTIFIER: &str = "#&#";
    pub const ENTRIES_SEPARATOR: &str = "\t";
}

pub struct SucessfulOperationConstants;

impl SucessfulOperationConstants {
    pub fn sucessful_insert_operation(key: &str, value: &str) -> String {
        format!("Sucessfully set the key {} with the value {} in the database!", key, value)
    }
    pub fn sucessful_delete_operation(key: &str) -> String {
        format!("Sucessfully deleted the entry for {}!", key)
    }
    pub fn sucessful_clear_operation() -> String {
        String::from("Sucessfully cleared all database values!")
    }
}