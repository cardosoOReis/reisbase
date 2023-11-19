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
                Some(old_value) => Err(CustomReisActionWarning::EntryAlreadyExists {
                    key: key.to_string(),
                    old_value,
                    new_value: new_value.to_string(),
                }),
                None => {
                    controller.database.insert(key, new_value);
                    Ok(CustomSuccessOperation::Insert(
                        SuccessfulOperationStrings::successful_insert_operation(key, new_value),
                    ))
                }
            },
            ReisbaseActions::Get { key, arguments } => {
                if let Some(value) = controller.database.get(key) {
                    if arguments.contains(&ReisbaseActionsArguments::Clipboard) {
                        if let Ok(mut clipboard) = Clipboard::new() {
                            let _ = clipboard.set_text(&value);
                        }
                    }
                    Ok(CustomSuccessOperation::Get(value))
                } else {
                    Err(CustomReisActionWarning::EntryDoesntExists {
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
                    Err(CustomReisActionWarning::EntryDoesntExists {
                        key: key.to_string(),
                        value: Some(value.to_string()),
                    })
                } else {
                    controller.database.insert(key, value);
                    Ok(CustomSuccessOperation::Put(
                        SuccessfulOperationStrings::successful_insert_operation(key, value),
                    ))
                }
            }
            ReisbaseActions::Del { key, arguments: _ } => controller
                .database
                .get(key)
                .map(|ref key| {
                    controller.database.delete(key);
                    CustomSuccessOperation::delete(key)
                })
                .ok_or_else(|| CustomReisActionWarning::entry_doesnt_exists(key, None)),
            ReisbaseActions::GetAll { arguments: _ } => controller
                .database
                .get_all()
                .map(CustomSuccessOperation::GetAll)
                .ok_or(CustomReisActionWarning::EmptyDatabase),
            ReisbaseActions::Clear { arguments } => {
                if controller.database.is_empty() {
                    return Err(CustomReisActionWarning::EmptyDatabase);
                }
                if arguments.contains(&ReisbaseActionsArguments::Force) {
                    controller.database.clear();
                    Ok(CustomSuccessOperation::Clear(
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
