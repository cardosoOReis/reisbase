use crate::controller::ReisbaseActions;
use std::io::Error;

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
    EmptyDatabaseWarning,
    EntryAlreadyExistsWarning {
        key: String,
        old_value: String,
        new_value: String,
    },
    EntryDoesntExistsWarning {
        key: String,
        value: Option<String>,
    },
    RequiredArgumentsNotSpecified {
        operation: ReisbaseActions,
    },
}

#[derive(Debug)]
pub enum CustomFailureOperation {
    Failure(CustomReisIOFailure),
    Warning(CustomReisActionWarning),
}
