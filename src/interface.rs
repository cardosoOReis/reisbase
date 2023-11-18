use std::io::{Error, ErrorKind};

use crate::{
    controller::Controller,
    error_handler::ErrorHandler,
    failures::{CustomFailureOperation, CustomReisIOFailure},
    sucess::CustomSucessOperation,
};

#[derive(Debug)]
pub struct Interface {
    controller: Controller,
}

impl Interface {
    fn new(
        action: &str,
        key: Option<String>,
        value: Option<String>,
        arguments: Option<Vec<String>>,
    ) -> Result<Interface, CustomReisIOFailure> {
        let controller = Controller::new(action, key, value, arguments);
        match controller {
            Ok(controller) => Ok(Interface { controller }),
            Err(error) => Err(error),
        }
    }

    pub fn execute(
        action: Option<String>,
        key: Option<String>,
        value: Option<String>,
        arguments: Vec<String>,
    ) -> Result<CustomSucessOperation, CustomFailureOperation> {
        match action {
            Some(action) => {
                match Self::new(&action, key, value, Some(arguments)) {
                    Ok(mut interface) => match interface.controller.execute() {
                        Ok(result) => Ok(result),
                        Err(warning) => Err(CustomFailureOperation::Warning(warning)),
                    },
                    Err(error) => Err(CustomFailureOperation::Failure(error)),
                }
            }
            None => {
                    Err(CustomFailureOperation::Failure(
                        ErrorHandler::handle_io_error(Error::new(
                            ErrorKind::InvalidInput,
                            "Operation should contain an action!",
                        )),
                    ))
                }
        }
    }
}
