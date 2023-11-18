#[derive(Debug)]
pub enum CustomSucessOperation {
    SucessInsertOperation(String),
    SucessGetOperation(String),
    SucessPutOperation(String),
    SucessDeleteOperation(String),
    SucessGetAllOperation(String),
    SucessClearOperation(String),
}