use crate::constants::SuccessfulOperationStrings;

#[derive(Debug)]
pub enum CustomSuccessOperation {
    Insert(String),
    Get(String),
    Put(String),
    Delete(String),
    GetAll(String),
    Clear(String),
}

impl CustomSuccessOperation {
    pub fn delete(key: &str) -> CustomSuccessOperation {
        CustomSuccessOperation::Delete(
            SuccessfulOperationStrings::successful_delete_operation(key),
        )
    }
    pub fn message(&self) -> &str {
        match self {
            CustomSuccessOperation::Insert(message) => message,
            CustomSuccessOperation::Get(message) => message,
            CustomSuccessOperation::Put(message) => message,
            CustomSuccessOperation::Delete(message) => message,
            CustomSuccessOperation::GetAll(message) => message,
            CustomSuccessOperation::Clear(message) => message,
        }
    }
}
