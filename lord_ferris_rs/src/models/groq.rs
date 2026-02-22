use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroqTextResponse {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroqResponse {
    pub output: Vec<GroqOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroqOutput {
    #[serde(rename = "type")]
    pub output_type: String,
    pub content: Option<Vec<GroqContent>>,
    pub summary: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroqContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}
