use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ChatRequest {
	pub system_instruction: Option<SystemInstruction>,
	pub contents: Vec<Content>,
}

#[derive(Debug, Serialize)]
pub struct SystemInstruction {
	pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
pub struct ContentResponse {
	pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct ChatResponse {
	pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamChunk {
	pub candidates: Vec<Candidate>,
	pub model_version: Option<String>,

	#[serde(rename = "createTime")]
	pub create_time: Option<String>,

	#[serde(rename = "responseId")]
	pub response_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Candidate {
	pub content: Content,

	#[serde(default)]
	#[serde(rename = "finishReason")]
	pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Content {
	pub role: String,
	pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Part {
	pub text: String,
}
