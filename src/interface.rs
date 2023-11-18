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
        arguments: Vec<String>,
    ) -> Result<Interface, CustomReisIOFailure> {
        Controller::new(action, key, value, arguments).map(|controller| Interface { controller })
    }

    pub fn execute(
        action: Option<String>,
        key: Option<String>,
        value: Option<String>,
        arguments: Vec<String>,
    ) -> Result<CustomSucessOperation, CustomFailureOperation> {
        action
            .ok_or_else(build_empty_action_error)
            .and_then(|action| create_interface_and_map_error(&action, key, value, arguments))
            .and_then(execute_action)
    }
}
fn create_interface_and_map_error(
    action: &str,
    key: Option<String>,
    value: Option<String>,
    arguments: Vec<String>,
) -> Result<Interface, CustomFailureOperation> {
    Interface::new(action, key, value, arguments).map_err(CustomFailureOperation::Failure)
}

fn build_empty_action_error() -> CustomFailureOperation {
    CustomFailureOperation::Failure(ErrorHandler::handle_io_error(Error::new(
        ErrorKind::InvalidInput,
        "Operation should contain an action!",
    )))
}

fn execute_action(
    mut interface: Interface,
) -> Result<CustomSucessOperation, CustomFailureOperation> {
    interface
        .controller
        .execute()
        .map_err(CustomFailureOperation::Warning)
}
