use std::env;
use std::io;
use strum::IntoEnumIterator;

use crate::actions::ReisbaseActions;
use crate::constants::the_entry_does_not_exists;
use crate::constants::the_key_already_exists;
use crate::constants::CANCELED_OPERATION;
use crate::constants::EMPTY_DATABASE;
use crate::constants::THIS_ACTION_IS_PERMANENT;
use crate::extensions::OptionFromPredicate;
use crate::operation::Operation;
use crate::{
    failures::{CustomFailureOperation, CustomReisActionWarning, CustomReisIOFailure},
    interface::Interface,
    sucess::CustomSucessOperation,
};

#[derive(Debug)]
pub struct TerminalCommunication;

impl TerminalCommunication {
    pub fn execute() {
        let operation = get_requested_operation();
        handle_result_interface_execute(Interface::execute(operation));
    }
}

fn handle_result_interface_execute(result: Result<CustomSucessOperation, CustomFailureOperation>) {
    match result {
        Ok(sucess_operation) => handle_sucess_operation(&sucess_operation),
        Err(failure_operation) => match failure_operation {
            CustomFailureOperation::Failure(failure) => handle_failure_operation(&failure),
            CustomFailureOperation::Warning(warning) => handle_warning_operation(&warning),
        },
    }
}

fn handle_sucess_operation(sucess: &CustomSucessOperation) {
    println!("{}", sucess.message());
}

fn handle_failure_operation(failure: &CustomReisIOFailure) {
    let error_message = failure.error_message();
    println!("{}", error_message);
    error_message.print_error();
}

fn handle_warning_operation(warning: &CustomReisActionWarning) {
    match warning {
        CustomReisActionWarning::EmptyDatabaseWarning => {
            println!("{}", EMPTY_DATABASE)
        }
        CustomReisActionWarning::EntryAlreadyExistsWarning {
            key,
            old_value,
            new_value,
        } => {
            retry(&the_key_already_exists(key, old_value), || {
                retry_put(key, new_value);
            });
        }
        CustomReisActionWarning::EntryDoesntExistsWarning { key, value } => {
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
    handle_result_interface_execute(Interface::execute(Some(Operation::clear())));
}

fn retry_put(key: &str, value: &str) {
    handle_result_interface_execute(Interface::execute(Some(Operation::put(key, value))));
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
        .find(|reisbase_action| has_same_name(action.as_deref(), reisbase_action))
        .and_then(|reisbase_action| parse_operation(&reisbase_action, action.as_deref(), args))
}

fn has_same_name(action: Option<&str>, reisbase_action: &ReisbaseActions) -> bool {
    let name = reisbase_action.name();
    action.map(|a| a == name.0 || a == name.1).unwrap_or(false)
}

fn parse_operation(
    reisbase_action: &ReisbaseActions,
    action: Option<&str>,
    mut args: impl Iterator<Item = String>,
) -> Option<Operation> {
    let key = Option::from_predicate(reisbase_action.has_key(), |_| args.next());
    let value = Option::from_predicate(reisbase_action.has_value(), |_| args.next());
    let args = args.collect::<Vec<String>>();
    Operation::new(action.map(|s| s.to_owned()), key, value, args)
}
