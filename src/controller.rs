use std::io::{Error, ErrorKind};

use crate::{
    actions::ReisbaseActions,
    arguments::ReisbaseActionsArguments,
    constants::DatabaseStringConstants,
    error_handler::ErrorHandler,
    failures::{CustomReisActionWarning, CustomReisIOFailure},
    reisbase::Reisbase,
    success::CustomSuccessOperation,
};

#[derive(Debug)]
pub struct Controller {
    pub action: ReisbaseActions,
    pub database: Reisbase,
}

impl Controller {
    pub fn new(
        action: &str,
        key: Option<String>,
        value: Option<String>,
        arguments: Vec<String>,
    ) -> Result<Controller, CustomReisIOFailure> {
        let arguments = arguments
            .iter()
            .filter_map(|argument| ReisbaseActionsArguments::new(argument))
            .collect();
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

    pub fn execute(&mut self) -> Result<CustomSuccessOperation, CustomReisActionWarning> {
        ReisbaseActions::execute(self)
    }
}
fn match_action(
    action: &str,
    key: Option<String>,
    value: Option<String>,
    arguments: Vec<ReisbaseActionsArguments>,
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
