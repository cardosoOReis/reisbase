use std::io::Error;

use crate::{actions::ReisbaseAction, arguments::ReisbaseActionsArguments};

#[derive(Debug)]
pub enum CustomReisIOFailure {
    CorruptedDatabase(CustomErrorMessage),
    DatabaseNotFound(CustomErrorMessage),
    DatabaseTooLarge(CustomErrorMessage),
    Default(CustomErrorMessage),
    InvalidActionArguments(CustomErrorMessage),
    InvalidDatabaseName(CustomErrorMessage),
    InvalidInput(CustomErrorMessage),
    InvalidPlatformOperation(CustomErrorMessage),
    PermissionDeniedForDatabase(CustomErrorMessage),
    OutOfSpace(CustomErrorMessage),
    UnknownActionRequest(CustomErrorMessage),
}

impl CustomReisIOFailure {
    pub fn error_message(&self) -> &CustomErrorMessage {
        match self {
            CustomReisIOFailure::CorruptedDatabase(error_message)
            | CustomReisIOFailure::DatabaseNotFound(error_message)
            | CustomReisIOFailure::DatabaseTooLarge(error_message)
            | CustomReisIOFailure::Default(error_message)
            | CustomReisIOFailure::InvalidDatabaseName(error_message)
            | CustomReisIOFailure::InvalidInput(error_message)
            | CustomReisIOFailure::InvalidPlatformOperation(error_message)
            | CustomReisIOFailure::PermissionDeniedForDatabase(error_message)
            | CustomReisIOFailure::OutOfSpace(error_message)
            | CustomReisIOFailure::InvalidActionArguments(error_message)
            | CustomReisIOFailure::UnknownActionRequest(error_message) => error_message,
        }
    }

    pub fn invalid_action_arguments(action_name: &str) -> CustomReisIOFailure {
        let message = format!(
            "Invalid arguments were passed for the {} action!",
            action_name
        );
        CustomReisIOFailure::InvalidActionArguments(CustomErrorMessage {
            message,
            error: Error::new(std::io::ErrorKind::InvalidInput, "See message"),
        })
    }

    pub fn unknown_action_requested(action: &str) -> CustomReisIOFailure {
        CustomReisIOFailure::UnknownActionRequest(CustomErrorMessage {
            message: format!(
                "The argument {} is not recongnized as a real action.",
                action
            ),
            error: Error::new(std::io::ErrorKind::InvalidInput, action),
        })
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
        operation: ReisbaseAction,
    },
}

impl CustomReisActionWarning {
    pub fn entry_already_exists(
        key: &str,
        old_value: &str,
        new_value: &str,
    ) -> CustomReisActionWarning {
        CustomReisActionWarning::EntryAlreadyExists {
            key: String::from(key),
            old_value: String::from(old_value),
            new_value: String::from(new_value),
        }
    }
    pub fn entry_doesnt_exists(key: &str, value: Option<&str>) -> CustomReisActionWarning {
        Self::EntryDoesntExists {
            key: String::from(key),
            value: value.map(String::from),
        }
    }
    pub fn clear_without_force() -> CustomReisActionWarning {
        Self::RequiredArgumentsNotSpecified {
            operation: ReisbaseAction::Clear {
                arguments: vec![ReisbaseActionsArguments::Force],
            },
        }
    }
}

#[derive(Debug)]
pub enum CustomFailureOperation {
    Error(CustomReisIOFailure),
    Warning(CustomReisActionWarning),
}
