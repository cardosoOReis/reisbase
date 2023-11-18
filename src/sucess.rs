#[derive(Debug)]
pub enum CustomSucessOperation {
    SucessInsertOperation(String),
    SucessGetOperation(String),
    SucessPutOperation(String),
    SucessDeleteOperation(String),
    SucessGetAllOperation(String),
    SucessClearOperation(String),
}

impl CustomSucessOperation {
    pub fn message(&self) -> &str {
        match self {
            CustomSucessOperation::SucessInsertOperation(message) => message,
            CustomSucessOperation::SucessGetOperation(message) => message,
            CustomSucessOperation::SucessPutOperation(message) => message,
            CustomSucessOperation::SucessDeleteOperation(message) => message,
            CustomSucessOperation::SucessGetAllOperation(message) => message,
            CustomSucessOperation::SucessClearOperation(message) => message,
        }
    }
}
