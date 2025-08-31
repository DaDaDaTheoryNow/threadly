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

// #[derive(Debug, Serialize)]
// pub struct Content {
// 	pub role: String,
// 	pub parts: Vec<Part>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Part {
// 	pub text: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct StreamChunk {
// 	pub candidates: Option<Vec<Candidate>>,
// }

// #[derive(Debug, Deserialize)]
// pub struct Candidate {
// 	pub content: ContentResponse,
// }

#[derive(Debug, Deserialize)]
pub struct ContentResponse {
	pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct ChatResponse {
	pub text: String,
}

// #[derive(Debug, serde::Deserialize)]
// pub struct StreamChunk {
// 	pub candidates: Option<Vec<Candidate>>,
// 	#[serde(flatten)]
// 	pub _extra: Option<serde_json::Value>, // остальные поля игнорируем
// }

// #[derive(Debug, serde::Deserialize, Serialize)]
// pub struct Candidate {
// 	pub content: Content,
// 	pub finishReason: Option<String>,
// }

// #[derive(Debug, serde::Deserialize, Serialize)]
// pub struct Content {
// 	pub role: String,
// 	pub parts: Vec<Part>,
// }

// #[derive(Debug, serde::Deserialize, Serialize)]
// pub struct Part {
// 	pub text: String,
// }

#[derive(Debug, Deserialize, Clone)]
pub struct StreamChunk {
	pub candidates: Vec<Candidate>,
	// pub usage_metadata: Option<UsageMetadata>,
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

// #[derive(Debug, Deserialize)]
// pub struct UsageMetadata {
// 	pub prompt_token_count: Option<u32>,
// 	pub candidates_token_count: Option<u32>,
// 	pub total_token_count: Option<u32>,
// 	pub traffic_type: Option<String>,
// }
