use crate::{
    actions::ReisbaseAction,
    arguments::ReisbaseActionsArguments,
    constants::DatabaseStringConstants,
    failures::{CustomReisActionWarning, CustomReisIOFailure},
    reisbase::Reisbase,
    success::CustomSuccessOperation,
};

#[derive(Debug)]
pub struct Controller {
    pub action: ReisbaseAction,
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

        let action = ReisbaseAction::new(action, key, value, arguments)?;
        let database = Reisbase::build(DatabaseStringConstants::DATABASE_NAME)?;
        Ok(Controller { action, database })
    }

    pub fn execute(&mut self) -> Result<CustomSuccessOperation, CustomReisActionWarning> {
        ReisbaseAction::execute(self)
    }
}
