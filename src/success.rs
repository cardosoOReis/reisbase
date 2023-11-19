#[derive(Debug)]
pub enum CustomSuccessOperation {
    SuccessInsertOperation(String),
    SuccessGetOperation(String),
    SuccessPutOperation(String),
    SuccessDeleteOperation(String),
    SuccessGetAllOperation(String),
    SuccessClearOperation(String),
}

impl CustomSuccessOperation {
    pub fn message(&self) -> &str {
        match self {
            CustomSuccessOperation::SuccessInsertOperation(message) => message,
            CustomSuccessOperation::SuccessGetOperation(message) => message,
            CustomSuccessOperation::SuccessPutOperation(message) => message,
            CustomSuccessOperation::SuccessDeleteOperation(message) => message,
            CustomSuccessOperation::SuccessGetAllOperation(message) => message,
            CustomSuccessOperation::SuccessClearOperation(message) => message,
        }
    }
}
