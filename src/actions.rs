use arboard::Clipboard;
use strum_macros::EnumIter;

use crate::{
    arguments::ReisbaseActionsArguments, constants::SucessfulOperationStrings,
    controller::Controller, failures::CustomReisActionWarning, sucess::CustomSucessOperation,
};

#[derive(Debug, EnumIter)]
pub enum ReisbaseActions {
    Set {
        key: String,
        value: String,
        arguments: Vec<ReisbaseActionsArguments>,
    },
    Get {
        key: String,
        arguments: Vec<ReisbaseActionsArguments>,
    },
    Put {
        key: String,
        value: String,
        arguments: Vec<ReisbaseActionsArguments>,
    },
    Del {
        key: String,
        arguments: Vec<ReisbaseActionsArguments>,
    },
    GetAll {
        arguments: Vec<ReisbaseActionsArguments>,
    },
    Clear {
        arguments: Vec<ReisbaseActionsArguments>,
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
            } => match controller.database.get(key) {
                Some(old_value) => Err(CustomReisActionWarning::EntryAlreadyExistsWarning {
                    key: key.to_string(),
                    old_value,
                    new_value: new_value.to_string(),
                }),
                None => {
                    controller.database.insert(key, new_value);
                    Ok(CustomSucessOperation::SucessInsertOperation(
                        SucessfulOperationStrings::sucessful_insert_operation(key, new_value),
                    ))
                }
            },
            ReisbaseActions::Get { key, arguments } => {
                if let Some(value) = controller.database.get(key) {
                    if has_specified_argument(&ReisbaseActionsArguments::Clipboard, arguments) {
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
                        SucessfulOperationStrings::sucessful_insert_operation(key, value),
                    ))
                }
            }
            ReisbaseActions::Del { key, arguments: _ } => {
                if controller.database.get(key).is_some() {
                    controller.database.delete(key);
                    Ok(CustomSucessOperation::SucessDeleteOperation(
                        SucessfulOperationStrings::sucessful_delete_operation(key),
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
                if has_specified_argument(&ReisbaseActionsArguments::Force, arguments) {
                    controller.database.clear();
                    Ok(CustomSucessOperation::SucessClearOperation(
                        SucessfulOperationStrings::sucessful_clear_operation(),
                    ))
                } else {
                    Err(CustomReisActionWarning::RequiredArgumentsNotSpecified {
                        operation: ReisbaseActions::Clear {
                            arguments: vec![ReisbaseActionsArguments::Force],
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
    specified_argument: &ReisbaseActionsArguments,
    arguments: &[ReisbaseActionsArguments],
) -> bool {
    arguments
        .iter()
        .any(|argument| argument == specified_argument)
}
