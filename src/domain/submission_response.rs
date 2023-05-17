use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorField {
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormErrors {
    #[serde(rename = "FieldErrors")]
    pub field_errors: std::collections::HashMap<String, Vec<ErrorField>>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionResponse {
    #[serde(rename = "HasErrors")]
    pub has_errors: bool,
    #[serde(rename = "FormErrors")]
    pub form_errors: Option<FormErrors>,
}
