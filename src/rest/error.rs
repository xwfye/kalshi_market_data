use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorData{
    pub code: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorMessage{
    pub error: ErrorData
}