use std::env;
use std::io;
use strum::IntoEnumIterator;

use crate::actions::ReisbaseActions;
use crate::extensions::OptionFromPredicate;
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
            let value = value.as_deref().unwrap_or("value");
            println!("The entry {} does not exists! You can create a new one with the command: set {} {}", key, key, value);
        }
        CustomReisActionWarning::RequiredArgumentsNotSpecified { operation } => {
            if let ReisbaseActions::Clear { arguments: _ } = operation {
                match get_user_input("This action is permanent, and will clear all your data. Are you sure you want to continue? (Y/n)") {
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
    let mut args = env::args().skip(1);
    let action = args.next();
    ReisbaseActions::iter()
        .find(|reisbase_action| has_same_name(action.as_deref(), reisbase_action))
        .map(|reisbase_action| parse_operation(&reisbase_action, action.as_deref(), args))
        .unwrap_or_else(|| (action, None, None, Vec::new()))
}

fn has_same_name(action: Option<&str>, reisbase_action: &ReisbaseActions) -> bool {
    let name = reisbase_action.name();
    action.map(|a| a == name.0 || a == name.1).unwrap_or(false)
}

fn parse_operation(
    reisbase_action: &ReisbaseActions,
    action: Option<&str>,
    mut args: impl Iterator<Item = String>,
) -> (Option<String>, Option<String>, Option<String>, Vec<String>) {
    let key = Option::from_predicate(reisbase_action.has_key(), |_| args.next());
    let value = Option::from_predicate(reisbase_action.has_value(), |_| args.next());
    let args = args.collect::<Vec<String>>();
    (action.map(|s| s.to_owned()), key, value, args)
}
