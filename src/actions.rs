use arboard::Clipboard;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    arguments::ReisbaseActionsArguments,
    controller::Controller,
    extensions::{PeekOption, ResultFromPredicate},
    failures::{CustomReisActionWarning, CustomReisIOFailure},
    success::CustomSuccessOperation,
};

#[derive(Debug, EnumIter)]
pub enum ReisbaseAction {
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

impl ReisbaseAction {
    pub fn execute(
        controller: &mut Controller,
    ) -> Result<CustomSuccessOperation, CustomReisActionWarning> {
        match &controller.action {
            ReisbaseAction::Set {
                key,
                value: new_value,
                arguments: _,
            } => match controller.database.get(key) {
                Some(ref old_value) => Err(CustomReisActionWarning::entry_already_exists(
                    key, old_value, new_value,
                )),
                None => {
                    controller.database.insert(key, new_value);
                    Ok(CustomSuccessOperation::insert(key, new_value))
                }
            },
            ReisbaseAction::Get { key, arguments } => controller
                .database
                .get(key)
                .peek(|value| {
                    if arguments.contains(&ReisbaseActionsArguments::Clipboard) {
                        text_to_clipboard(value);
                    }
                })
                .map(CustomSuccessOperation::Get)
                .ok_or_else(|| CustomReisActionWarning::entry_doesnt_exists(key, None)),
            ReisbaseAction::Put {
                key,
                value,
                arguments: _,
            } => Result::from_predicate(
                controller.database.exists(key),
                || {
                    controller.database.insert(key, value);
                    CustomSuccessOperation::put(key, value)
                },
                || CustomReisActionWarning::entry_doesnt_exists(key, Some(value)),
            ),
            ReisbaseAction::Del { key, arguments: _ } => Result::from_predicate(
                controller.database.exists(key),
                || {
                    controller.database.delete(key);
                    CustomSuccessOperation::delete(key)
                },
                || CustomReisActionWarning::entry_doesnt_exists(key, None),
            ),
            ReisbaseAction::GetAll { arguments: _ } => controller
                .database
                .get_all()
                .map(CustomSuccessOperation::GetAll)
                .ok_or(CustomReisActionWarning::EmptyDatabase),
            ReisbaseAction::Clear { arguments } => {
                if controller.database.is_empty() {
                    return Err(CustomReisActionWarning::EmptyDatabase);
                }
                Result::from_predicate(
                    arguments.contains(&ReisbaseActionsArguments::Force),
                    || {
                        controller.database.clear();
                        CustomSuccessOperation::clear()
                    },
                    CustomReisActionWarning::clear_without_force,
                )
            }
        }
    }

    pub fn new(
        action: &str,
        key: Option<String>,
        value: Option<String>,
        arguments: Vec<ReisbaseActionsArguments>,
    ) -> Result<ReisbaseAction, CustomReisIOFailure> {
        let reisbase_action = ReisbaseAction::first(action)
            .ok_or_else(|| CustomReisIOFailure::unknown_action_requested(action))?;
        let action_name = reisbase_action.action_name();
        let action = match reisbase_action {
            ReisbaseAction::Set { .. } => {
                let (key, value) = parse_key_and_value(key, value, action_name)?;
                ReisbaseAction::Set {
                    key,
                    value,
                    arguments,
                }
            }
            ReisbaseAction::Get { .. } => {
                let key = parse_key_or_value(key, action_name)?;
                ReisbaseAction::Get { key, arguments }
            }
            ReisbaseAction::Put { .. } => {
                let (key, value) = parse_key_and_value(key, value, action_name)?;
                ReisbaseAction::Put {
                    key,
                    value,
                    arguments,
                }
            }
            ReisbaseAction::Del { .. } => {
                let key = parse_key_or_value(key, action_name)?;
                ReisbaseAction::Del { key, arguments }
            }
            ReisbaseAction::GetAll { .. } => ReisbaseAction::GetAll { arguments },
            ReisbaseAction::Clear { .. } => ReisbaseAction::Clear { arguments },
        };

        Ok(action)
    }

    pub fn action_name(&self) -> &str {
        match self {
            ReisbaseAction::Set { .. } => "Set",
            ReisbaseAction::Get { .. } => "Get",
            ReisbaseAction::Put { .. } => "Put",
            ReisbaseAction::Del { .. } => "Delete",
            ReisbaseAction::GetAll { .. } => "Get All",
            ReisbaseAction::Clear { .. } => "Clear",
        }
    }

    pub fn first(action: &str) -> Option<Self> {
        ReisbaseAction::iter().find(|ac| ac.has_same_name(action))
    }

    pub fn names(&self) -> &[&str] {
        match self {
            ReisbaseAction::Set {
                key: _,
                value: _,
                arguments: _,
            } => &["s", "set"],
            ReisbaseAction::Get {
                key: _,
                arguments: _,
            } => &["g", "get"],
            ReisbaseAction::Put {
                key: _,
                value: _,
                arguments: _,
            } => &["p", "put"],
            ReisbaseAction::Del {
                key: _,
                arguments: _,
            } => &["d", "del"],
            ReisbaseAction::GetAll { arguments: _ } => &["ga", "getall"],
            ReisbaseAction::Clear { arguments: _ } => &["c", "clr"],
        }
    }

    pub fn has_same_name(&self, action: &str) -> bool {
        self.names().contains(&action)
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
            ReisbaseAction::Set { .. } => true,
            ReisbaseAction::Get { .. } => true,
            ReisbaseAction::Put { .. } => true,
            ReisbaseAction::Del { .. } => true,
            ReisbaseAction::GetAll { .. } => false,
            ReisbaseAction::Clear { .. } => false,
        }
    }
    fn has_value(&self) -> bool {
        match self {
            ReisbaseAction::Set { .. } => true,
            ReisbaseAction::Get { .. } => false,
            ReisbaseAction::Put { .. } => true,
            ReisbaseAction::Del { .. } => false,
            ReisbaseAction::GetAll { .. } => false,
            ReisbaseAction::Clear { .. } => false,
        }
    }
}

fn text_to_clipboard(value: &str) {
    if let Ok(mut clipboard) = Clipboard::new() {
        _ = clipboard.set_text(value);
    }
}

fn parse_key_or_value(s: Option<String>, action_name: &str) -> Result<String, CustomReisIOFailure> {
    s.ok_or_else(|| CustomReisIOFailure::invalid_action_arguments(action_name))
}

fn parse_key_and_value(
    key: Option<String>,
    value: Option<String>,
    action_name: &str,
) -> Result<(String, String), CustomReisIOFailure> {
    let key = parse_key_or_value(key, action_name)?;
    let value = parse_key_or_value(value, action_name)?;
    Ok((key, value))
}
