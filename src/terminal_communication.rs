use std::env;
use std::io;
use strum::IntoEnumIterator;

use crate::actions::ReisbaseActions;
use crate::constants::{
    the_entry_does_not_exists, the_key_already_exists, CANCELED_OPERATION, EMPTY_DATABASE,
    THIS_ACTION_IS_PERMANENT,
};
use crate::operation::Operation;
use crate::{
    failures::{CustomFailureOperation, CustomReisActionWarning, CustomReisIOFailure},
    interface::Interface,
    success::CustomSuccessOperation,
};

#[derive(Debug)]
pub struct TerminalCommunication;

impl TerminalCommunication {
    pub fn execute() {
        let operation = get_requested_operation();
        handle_interface_execution(operation);
    }
}

fn handle_interface_execution(operation: Option<Operation>) {
    let result = Interface::execute(operation);
    match result {
        Ok(operation) => handle_success_operation(&operation),
        Err(operation) => handle_failure_operation(&operation),
    };
}

fn handle_success_operation(success: &CustomSuccessOperation) {
    println!("{}", success.message());
}

fn handle_failure_operation(failure: &CustomFailureOperation) {
    match failure {
        CustomFailureOperation::Error(error) => handle_error_operation(error),
        CustomFailureOperation::Warning(warning) => handle_warning_operation(warning),
    }
}

fn handle_error_operation(error: &CustomReisIOFailure) {
    let error_message = error.error_message();
    println!("{}", error_message);
    error_message.print_error();
}

fn handle_warning_operation(warning: &CustomReisActionWarning) {
    match warning {
        CustomReisActionWarning::EmptyDatabase => {
            println!("{}", EMPTY_DATABASE)
        }
        CustomReisActionWarning::EntryAlreadyExists {
            key,
            old_value,
            new_value,
        } => {
            retry(&the_key_already_exists(key, old_value), || {
                retry_put(key, new_value);
            });
        }
        CustomReisActionWarning::EntryDoesntExists { key, value } => {
            let value = value.as_deref().unwrap_or("value");
            println!("{}", the_entry_does_not_exists(key, value));
        }
        CustomReisActionWarning::RequiredArgumentsNotSpecified { operation } => {
            if let ReisbaseActions::Clear { arguments: _ } = operation {
                retry(THIS_ACTION_IS_PERMANENT, retry_clear);
            }
        }
    }
}

fn retry<F: FnOnce()>(prompt: &str, retry_f: F) {
    let input = get_user_input(prompt).map(|input| user_input_to_bool(&input));
    match input {
        Ok(true) => retry_f(),
        Ok(false) => println!("{}", CANCELED_OPERATION),
        Err(error) => handle_retry_error(&error),
    };
}

fn retry_clear() {
    handle_interface_execution(Some(Operation::clear()));
}

fn retry_put(key: &str, value: &str) {
    handle_interface_execution(Some(Operation::put(key, value)));
}

fn handle_retry_error(error: &io::Error) {
    println!("Sorry, an error occured when attempting to read your input!");
    eprintln!("{}", error);
}

fn get_user_input(prompt: &str) -> Result<String, std::io::Error> {
    println!("{prompt}");
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    Ok(response)
}

fn user_input_to_bool(input: &str) -> bool {
    input.to_lowercase().starts_with('y')
}

fn get_requested_operation() -> Option<Operation> {
    let mut args = env::args().skip(1);
    let action = args.next();
    ReisbaseActions::iter()
        .find(|reisbase_action| reisbase_action.has_same_name(action.as_deref()))
        .and_then(|reisbase_action| parse_action(&reisbase_action, action.as_deref(), args))
}

fn parse_action(
    reisbase_action: &ReisbaseActions,
    action: Option<&str>,
    mut args: impl Iterator<Item = String>,
) -> Option<Operation> {
    let key = reisbase_action.with_key(|| args.next());
    let value = reisbase_action.with_value(|| args.next());
    let args = args.collect::<Vec<String>>();
    Operation::new(action.map(|s| s.to_owned()), key, value, args)
}
