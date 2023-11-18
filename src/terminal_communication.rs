use std::env;
use std::io;
use strum::IntoEnumIterator;

use crate::controller::ReisbaseActions;
use crate::{
    failures::{CustomFailureOperation, CustomReisActionWarning, CustomReisIOFailure},
    interface::Interface,
    sucess::CustomSucessOperation,
};

#[derive(Debug)]
pub struct TerminalCommunication;

impl TerminalCommunication {
    pub fn execute() {
        let user_input = get_requested_operation();
        handle_result_interface_execute(Interface::execute(
            user_input.0,
            user_input.1,
            user_input.2,
            user_input.3,
        ));
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
    match sucess {
        CustomSucessOperation::SucessInsertOperation(message)
        | CustomSucessOperation::SucessGetOperation(message)
        | CustomSucessOperation::SucessPutOperation(message)
        | CustomSucessOperation::SucessDeleteOperation(message)
        | CustomSucessOperation::SucessGetAllOperation(message)
        | CustomSucessOperation::SucessClearOperation(message) => println!("{}", message),
    }
}

fn handle_failure_operation(failure: &CustomReisIOFailure) {
    match failure {
        CustomReisIOFailure::CorruptedDatabaseFailure(error_message)
        | CustomReisIOFailure::DatabaseNotFoundFailure(error_message)
        | CustomReisIOFailure::DatabaseTooLargeError(error_message)
        | CustomReisIOFailure::DefaultReisFailure(error_message)
        | CustomReisIOFailure::InvalidDatabaseNameFailure(error_message)
        | CustomReisIOFailure::InvalidInputFailure(error_message)
        | CustomReisIOFailure::InvalidPlatformOperationFailure(error_message)
        | CustomReisIOFailure::PermissionDeniedForDatabase(error_message)
        | CustomReisIOFailure::OutOfSpaceFailure(error_message)
        | CustomReisIOFailure::UnknownOperationFailure(error_message) => {
            println!("{}", error_message);
            error_message.print_error()
        }
    }
}

fn handle_warning_operation(warning: &CustomReisActionWarning) {
    match warning {
        CustomReisActionWarning::EmptyDatabaseWarning => {
            println!("Database doesn't contain any value!")
        }
        CustomReisActionWarning::EntryAlreadyExistsWarning {
            key,
            old_value,
            new_value,
        } => {
            let response = get_user_input(&format!("The key {} already exists in this database, with the value of {}. Do you want to replace it? (Y/n)", key, old_value));
            match response {
                Ok(response) => {
                    if response.to_lowercase().starts_with('y') {
                        handle_result_interface_execute(Interface::execute(
                            Some(String::from("put")),
                            Some(key.to_owned()),
                            Some(new_value.to_owned()),
                            Vec::new(),
                        ))
                    }
                }
                Err(error) => {
                    println!("Sorry, an error occured when attempting to read your input!");
                    eprintln!("{}", error);
                }
            }
        }
        CustomReisActionWarning::EntryDoesntExistsWarning { key, value } => {
            if let Some(value) = value {
                println!("The entry {} does not exists! You can create a new one with the command: set {} {}", key, key, value)
            } else {
                println!("The entry {} does not exists! You can create a new one with the command: set {} value", key, key)
            }
        }
        CustomReisActionWarning::RequiredArgumentsNotSpecified { operation } => {
            if let ReisbaseActions::Clear { arguments: _ } = operation {
                match get_user_input("This action is permanent, and will clear all your data! Are you sure you want to continue? (Y/n)") {
                    Ok(input) => {
                        if parse_user_input_to_bool(&input) {
                            handle_result_interface_execute(Interface:: execute(
                                Some(String::from("c")), 
                                None, 
                                None,
                                vec![String::from("-f")],),);
                        } else {
                            println!("The operation was canceled!");
                        }
                    }
                    Err(error) => {
                        println!("Sorry, an error occured when attempting to read your input!");
                        eprintln!("{}", error);
                    },
                }
            }
        }
    }
}

fn get_user_input(prompt: &str) -> Result<String, std::io::Error> {
    println!("{prompt}");
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    Ok(response)
}

fn parse_user_input_to_bool(input: &str) -> bool {
    input.starts_with('y') || input.starts_with('Y')
}

fn get_requested_operation() -> (Option<String>, Option<String>, Option<String>, Vec<String>) {
    for reisbase_action in ReisbaseActions::iter() {
        let args = &mut env::args().skip(1);
        let action = args.next();
        let name = reisbase_action.name();
        if let Some(action) = action {
            if action == name.0 || action == name.1 {
                let mut key: Option<String> = None;
                let mut value: Option<String> = None;
                if reisbase_action.has_key() {
                    key = args.next();
                }
                if reisbase_action.has_value() {
                    value = args.next();
                }
                let arguments: Vec<String> = args.collect();
                return (Some(action), key, value, arguments);
            }
        }
    }
    (env::args().nth(1), None, None, Vec::new())
}
