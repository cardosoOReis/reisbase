use std::io::Error;

use crate::actions::ReisbaseActions;

#[derive(Debug)]
pub enum CustomReisIOFailure {
    CorruptedDatabaseFailure(CustomErrorMessage),
    DatabaseNotFoundFailure(CustomErrorMessage),
    DatabaseTooLargeError(CustomErrorMessage),
    DefaultReisFailure(CustomErrorMessage),
    InvalidDatabaseNameFailure(CustomErrorMessage),
    InvalidInputFailure(CustomErrorMessage),
    InvalidPlatformOperationFailure(CustomErrorMessage),
    PermissionDeniedForDatabase(CustomErrorMessage),
    OutOfSpaceFailure(CustomErrorMessage),
    UnknownOperationFailure(CustomErrorMessage),
}

impl CustomReisIOFailure {
    pub fn error_message(&self) -> &CustomErrorMessage {
        match self {
            CustomReisIOFailure::CorruptedDatabaseFailure(error_message)
            | CustomReisIOFailure::DatabaseNotFoundFailure(error_message)
            | CustomReisIOFailure::DatabaseTooLargeError(error_message)
            | CustomReisIOFailure::DefaultReisFailure(error_message)
            | CustomReisIOFailure::InvalidDatabaseNameFailure(error_message)
            | CustomReisIOFailure::InvalidInputFailure(error_message)
            | CustomReisIOFailure::InvalidPlatformOperationFailure(error_message)
            | CustomReisIOFailure::PermissionDeniedForDatabase(error_message)
            | CustomReisIOFailure::OutOfSpaceFailure(error_message)
            | CustomReisIOFailure::UnknownOperationFailure(error_message) => error_message,
        }
    }
}

#[derive(Debug)]
pub struct CustomErrorMessage {
    message: String,
    error: Error,
}

impl std::fmt::Display for CustomErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message())
    }
}

impl CustomErrorMessage {
    pub fn new(message: String, error: Error) -> CustomErrorMessage {
        CustomErrorMessage { message, error }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn error(&self) -> &Error {
        &self.error
    }
    pub fn print_error(&self) {
        eprintln!("{}", self.error);
    }
}

#[derive(Debug)]
pub enum CustomReisActionWarning {
    EmptyDatabase,
    EntryAlreadyExists {
        key: String,
        old_value: String,
        new_value: String,
    },
    EntryDoesntExists {
        key: String,
        value: Option<String>,
    },
    RequiredArgumentsNotSpecified {
        operation: ReisbaseActions,
    },
}

impl CustomReisActionWarning {
    pub fn entry_doesnt_exists(key: &str, value: Option<&str>) -> CustomReisActionWarning {
        Self::EntryDoesntExists {
            key: String::from(key),
            value: value.map(String::from),
        }
    }
}

#[derive(Debug)]
pub enum CustomFailureOperation {
    Error(CustomReisIOFailure),
    Warning(CustomReisActionWarning),
}
