use arboard::Clipboard;
use strum_macros::EnumIter;

use crate::{
    arguments::ReisbaseActionsArguments, constants::SuccessfulOperationStrings,
    controller::Controller, failures::CustomReisActionWarning, success::CustomSuccessOperation,
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
    ) -> Result<CustomSuccessOperation, CustomReisActionWarning> {
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
                    Ok(CustomSuccessOperation::SuccessInsertOperation(
                        SuccessfulOperationStrings::successful_insert_operation(key, new_value),
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
                    Ok(CustomSuccessOperation::SuccessGetOperation(value))
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
                    Ok(CustomSuccessOperation::SuccessPutOperation(
                        SuccessfulOperationStrings::successful_insert_operation(key, value),
                    ))
                }
            }
            ReisbaseActions::Del { key, arguments: _ } => {
                if controller.database.get(key).is_some() {
                    controller.database.delete(key);
                    Ok(CustomSuccessOperation::SuccessDeleteOperation(
                        SuccessfulOperationStrings::successful_delete_operation(key),
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
                    Ok(CustomSuccessOperation::SuccessGetAllOperation(values))
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
                    Ok(CustomSuccessOperation::SuccessClearOperation(
                        SuccessfulOperationStrings::successful_clear_operation(),
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

    pub fn names(&self) -> &[&str] {
        match self {
            ReisbaseActions::Set {
                key: _,
                value: _,
                arguments: _,
            } => &["s", "set"],
            ReisbaseActions::Get {
                key: _,
                arguments: _,
            } => &["g", "get"],
            ReisbaseActions::Put {
                key: _,
                value: _,
                arguments: _,
            } => &["p", "put"],
            ReisbaseActions::Del {
                key: _,
                arguments: _,
            } => &["d", "del"],
            ReisbaseActions::GetAll { arguments: _ } => &["ga", "getall"],
            ReisbaseActions::Clear { arguments: _ } => &["c", "clr"],
        }
    }

    pub fn has_same_name(&self, action: Option<&str>) -> bool {
        action.map(|a| self.names().contains(&a)).unwrap_or(false)
    }

    /// Returns the result of calling `f` if this action has a key. Otherwise returns [`None`]
    pub fn with_key<F>(&self, f: F) -> Option<String>
    where
        F: FnOnce() -> Option<String>,
    {
        if self.has_key() {
            f()
        } else {
            None
        }
    }
    
    /// Returns the result of calling `f` if this action has a value. Otherwise returns [`None`]
    pub fn with_value<F>(&self, f: F) -> Option<String>
    where
        F: FnOnce() -> Option<String>,
    {
        if self.has_value() {
            f()
        } else {
            None
        }
    }

    fn has_key(&self) -> bool {
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
    fn has_value(&self) -> bool {
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
