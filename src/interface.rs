use std::io::{Error, ErrorKind};

use crate::{
    controller::Controller,
    error_handler::ErrorHandler,
    failures::{CustomFailureOperation, CustomReisIOFailure},
    operation::Operation,
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
        operation: Option<Operation>,
    ) -> Result<CustomSucessOperation, CustomFailureOperation> {
        operation
            .ok_or_else(build_empty_action_error)
            .and_then(create_interface_and_map_error)
            .and_then(execute_action)
    }
}
fn create_interface_and_map_error(
    operation: Operation,
) -> Result<Interface, CustomFailureOperation> {
    Interface::new(
        &operation.action,
        operation.key,
        operation.value,
        operation.arguments,
    )
    .map_err(CustomFailureOperation::Failure)
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
