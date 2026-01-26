use serde::{Deserialize, Serialize};

/// pub struct event
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvaluationRequest {
    pub question: String,
    pub answer: String,
    pub interview_type: String,
}
