use arboard::Clipboard;
use std::io::{Error, ErrorKind};
use strum_macros::EnumIter;

use crate::{
    constants::{DatabaseStringConstants, SucessfulOperationConstants},
    error_handler::ErrorHandler,
    failures::{CustomReisActionWarning, CustomReisIOFailure},
    reisbase::Reisbase,
    sucess::CustomSucessOperation,
};

#[derive(Debug)]
pub struct Controller {
    action: ReisbaseActions,
    database: Reisbase,
}

impl Controller {
    pub fn new(
        action: &str,
        key: Option<String>,
        value: Option<String>,
        arguments: Option<Vec<String>>,
    ) -> Result<Controller, CustomReisIOFailure> {
        let arguments = reisbase_actions_arguments_from_string(arguments);
        match match_action(action, key, value, arguments) {
            Err(error) => Err(error),
            Ok(option_action) => match option_action {
                None => Err(ErrorHandler::handle_io_error(Error::new(
                    ErrorKind::InvalidInput,
                    "Unknown action requested!".to_string(),
                ))),
                Some(action) => match Reisbase::build(DatabaseStringConstants::DATABASE_NAME) {
                    Err(error) => Err(error),
                    Ok(database) => Ok(Controller { action, database }),
                },
            },
        }
    }

    pub fn execute(&mut self) -> Result<CustomSucessOperation, CustomReisActionWarning> {
        ReisbaseActions::execute(self)
    }
}
fn match_action(
    action: &str,
    key: Option<String>,
    value: Option<String>,
    arguments: Option<Vec<ReisbaseActionsArguments>>,
) -> Result<Option<ReisbaseActions>, CustomReisIOFailure> {
    match action {
        "set" | "s" => {
            if let Some(key) = key {
                if let Some(value) = value {
                    return Ok(Some(ReisbaseActions::Set {
                        key,
                        value,
                        arguments,
                    }));
                }
            }
            Err(ErrorHandler::handle_io_error(Error::new(
                ErrorKind::InvalidInput,
                "Invalid arguments were passed for the Set action!",
            )))
        }
        "get" | "g" => {
            if let Some(key) = key {
                return Ok(Some(ReisbaseActions::Get { key, arguments }));
            }
            Err(ErrorHandler::handle_io_error(Error::new(
                ErrorKind::InvalidInput,
                "Invalid arguments were passed for the Get action!",
            )))
        }
        "put" | "p" => {
            if let Some(key) = key {
                if let Some(value) = value {
                    return Ok(Some(ReisbaseActions::Put {
                        key,
                        value,
                        arguments,
                    }));
                }
            }
            Err(ErrorHandler::handle_io_error(Error::new(
                ErrorKind::InvalidInput,
                "Invalid arguments passed for the Put action!",
            )))
        }
        "del" | "d" => {
            if let Some(key) = key {
                return Ok(Some(ReisbaseActions::Del { key, arguments }));
            }
            Err(ErrorHandler::handle_io_error(Error::new(
                ErrorKind::InvalidInput,
                "Invalid arguments passed for the Set action!",
            )))
        }
        "getall" | "ga" => Ok(Some(ReisbaseActions::GetAll { arguments })),
        "clear" | "c" => Ok(Some(ReisbaseActions::Clear { arguments })),
        _ => Ok(None),
    }
}

#[derive(Debug, EnumIter)]
pub enum ReisbaseActions {
    Set {
        key: String,
        value: String,
        arguments: Option<Vec<ReisbaseActionsArguments>>,
    },
    Get {
        key: String,
        arguments: Option<Vec<ReisbaseActionsArguments>>,
    },
    Put {
        key: String,
        value: String,
        arguments: Option<Vec<ReisbaseActionsArguments>>,
    },
    Del {
        key: String,
        arguments: Option<Vec<ReisbaseActionsArguments>>,
    },
    GetAll {
        arguments: Option<Vec<ReisbaseActionsArguments>>,
    },
    Clear {
        arguments: Option<Vec<ReisbaseActionsArguments>>,
    },
}

impl ReisbaseActions {
    pub fn execute(
        controller: &mut Controller,
    ) -> Result<CustomSucessOperation, CustomReisActionWarning> {
        match &controller.action {
            ReisbaseActions::Set {
                key,
                value: new_value,
                arguments: _,
            } => {
                if let Some(old_value) = controller.database.get(key) {
                    Err(CustomReisActionWarning::EntryAlreadyExistsWarning {
                        key: key.to_string(),
                        old_value,
                        new_value: new_value.to_string(),
                    })
                } else {
                    controller.database.insert(key, new_value);
                    Ok(CustomSucessOperation::SucessInsertOperation(
                        SucessfulOperationConstants::sucessful_insert_operation(key, new_value),
                    ))
                }
            }
            ReisbaseActions::Get { key, arguments } => {
                if let Some(value) = controller.database.get(key) {
                    if has_specified_argument(ReisbaseActionsArguments::Clipboard, arguments) {
                        if let Ok(mut clipboard) = Clipboard::new() {
                            let _ = clipboard.set_text(&value);
                        }
                    }
                    Ok(CustomSucessOperation::SucessGetOperation(value))
                } else {
                    Err(CustomReisActionWarning::EntryDoesntExistsWarning {
                        key: key.to_string(),
                        value: None,
                    })
                }
            }
            ReisbaseActions::Put {
                key,
                value,
                arguments: _,
            } => {
                if controller.database.get(key).is_none() {
                    Err(CustomReisActionWarning::EntryDoesntExistsWarning {
                        key: key.to_string(),
                        value: Some(value.to_string()),
                    })
                } else {
                    controller.database.insert(key, value);
                    Ok(CustomSucessOperation::SucessPutOperation(
                        SucessfulOperationConstants::sucessful_insert_operation(key, value),
                    ))
                }
            }
            ReisbaseActions::Del { key, arguments: _ } => {
                if controller.database.get(key).is_some() {
                    controller.database.delete(key);
                    Ok(CustomSucessOperation::SucessDeleteOperation(
                        SucessfulOperationConstants::sucessful_delete_operation(key),
                    ))
                } else {
                    Err(CustomReisActionWarning::EntryDoesntExistsWarning {
                        key: key.to_string(),
                        value: None,
                    })
                }
            }
            ReisbaseActions::GetAll { arguments: _ } => {
                if let Some(values) = controller.database.get_all() {
                    Ok(CustomSucessOperation::SucessGetAllOperation(values))
                } else {
                    Err(CustomReisActionWarning::EmptyDatabaseWarning)
                }
            }
            ReisbaseActions::Clear { arguments } => {
                if controller.database.is_empty() {
                    return Err(CustomReisActionWarning::EmptyDatabaseWarning);
                }
                if has_specified_argument(ReisbaseActionsArguments::Force, arguments) {
                    controller.database.clear();
                    Ok(CustomSucessOperation::SucessClearOperation(
                        SucessfulOperationConstants::sucessful_clear_operation(),
                    ))
                } else {
                    Err(CustomReisActionWarning::RequiredArgumentsNotSpecified {
                        operation: ReisbaseActions::Clear {
                            arguments: Some(vec![ReisbaseActionsArguments::Force]),
                        },
                    })
                }
            }
        }
    }

    pub fn name(&self) -> (&str, &str) {
        match &self {
            ReisbaseActions::Set {
                key: _,
                value: _,
                arguments: _,
            } => ("s", "set"),
            ReisbaseActions::Get {
                key: _,
                arguments: _,
            } => ("g", "get"),
            ReisbaseActions::Put {
                key: _,
                value: _,
                arguments: _,
            } => ("p", "put"),
            ReisbaseActions::Del {
                key: _,
                arguments: _,
            } => ("d", "del"),
            ReisbaseActions::GetAll { arguments: _ } => ("ga", "getall"),
            ReisbaseActions::Clear { arguments: _ } => ("c", "clr"),
        }
    }
    pub fn has_key(&self) -> bool {
        match self {
            ReisbaseActions::Set {
                key: _,
                value: _,
                arguments: _,
            } => true,
            ReisbaseActions::Get {
                key: _,
                arguments: _,
            } => true,
            ReisbaseActions::Put {
                key: _,
                value: _,
                arguments: _,
            } => true,
            ReisbaseActions::Del {
                key: _,
                arguments: _,
            } => true,
            ReisbaseActions::GetAll { arguments: _ } => false,
            ReisbaseActions::Clear { arguments: _ } => false,
        }
    }
    pub fn has_value(&self) -> bool {
        match self {
            ReisbaseActions::Set {
                key: _,
                value: _,
                arguments: _,
            } => true,
            ReisbaseActions::Get {
                key: _,
                arguments: _,
            } => false,
            ReisbaseActions::Put {
                key: _,
                value: _,
                arguments: _,
            } => true,
            ReisbaseActions::Del {
                key: _,
                arguments: _,
            } => false,
            ReisbaseActions::GetAll { arguments: _ } => false,
            ReisbaseActions::Clear { arguments: _ } => false,
        }
    }
}
fn has_specified_argument(
    specified_argument: ReisbaseActionsArguments,
    arguments: &Option<Vec<ReisbaseActionsArguments>>,
) -> bool {
    match arguments {
        None => false,
        Some(arguments) => {
            arguments
                .iter()
                .filter(|argument| {
                    std::mem::discriminant(argument.to_owned())
                        == std::mem::discriminant(&specified_argument)
                })
                .count()
                > 0
        }
    }
}

#[derive(Debug)]
pub enum ReisbaseActionsArguments {
    Force,
    Help,
    Clipboard,
    Description(String),
}

impl std::fmt::Display for ReisbaseActionsArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReisbaseActionsArguments::Force => write!(f, "-f (Force)"),
            ReisbaseActionsArguments::Help => write!(f, "-h (Help)"),
            ReisbaseActionsArguments::Clipboard => write!(f, "-c (Copy to Clipboard)"),
            ReisbaseActionsArguments::Description(_) => write!(f, "-d (Description)"),
        }
    }
}

fn reisbase_actions_arguments_from_string(
    actions: Option<Vec<String>>,
) -> Option<Vec<ReisbaseActionsArguments>> {
    match actions {
        Some(actions) => {
            let mut arguments: Vec<ReisbaseActionsArguments> = Vec::new();
            for action in actions {
                if action == "-f" {
                    arguments.push(ReisbaseActionsArguments::Force)
                }
                if action == "-h" {
                    arguments.push(ReisbaseActionsArguments::Help)
                }
                if action == "-c" {
                    arguments.push(ReisbaseActionsArguments::Clipboard)
                }
            }
            Some(arguments)
        }
        None => None,
    }
}
