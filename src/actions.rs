use arboard::Clipboard;
use strum_macros::EnumIter;

use crate::{
    arguments::ReisbaseActionsArguments,
    controller::Controller,
    extensions::{PeekOption, ResultFromPredicate},
    failures::CustomReisActionWarning,
    success::CustomSuccessOperation,
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
                Some(ref old_value) => Err(CustomReisActionWarning::entry_already_exists(
                    key, old_value, new_value,
                )),
                None => {
                    controller.database.insert(key, new_value);
                    Ok(CustomSuccessOperation::insert(key, new_value))
                }
            },
            ReisbaseActions::Get { key, arguments } => controller
                .database
                .get(key)
                .peek(|value| {
                    if arguments.contains(&ReisbaseActionsArguments::Clipboard) {
                        text_to_clipboard(value);
                    }
                })
                .map(CustomSuccessOperation::Get)
                .ok_or_else(|| CustomReisActionWarning::entry_doesnt_exists(key, None)),
            ReisbaseActions::Put {
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
            ReisbaseActions::Del { key, arguments: _ } => Result::from_predicate(
                controller.database.exists(key),
                || {
                    controller.database.delete(key);
                    CustomSuccessOperation::delete(key)
                },
                || CustomReisActionWarning::entry_doesnt_exists(key, None),
            ),
            ReisbaseActions::GetAll { arguments: _ } => controller
                .database
                .get_all()
                .map(CustomSuccessOperation::GetAll)
                .ok_or(CustomReisActionWarning::EmptyDatabase),
            ReisbaseActions::Clear { arguments } => {
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

fn text_to_clipboard(value: &str) {
    if let Ok(mut clipboard) = Clipboard::new() {
        let _ = clipboard.set_text(value);
    }
}
