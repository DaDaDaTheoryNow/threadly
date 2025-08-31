use std::sync::Arc;

use crate::error::{Error, Result};
use crate::{models::*, token_manager::TokenManager};
use dashmap::DashMap;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_streams::JsonStreamResponse;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct AiClient {
	http: Client,
	project: String,
	location: String,
	model: String,
	access_token: TokenManager,

	generating_sessions: Arc<DashMap<Uuid, ()>>,
}

impl AiClient {
	pub fn new(project: &str, location: &str, model: &str, key_path: &str) -> Self {
		Self {
			http: Client::new(),
			project: project.to_string(),
			location: location.to_string(),
			model: model.to_string(),
			access_token: TokenManager::new(key_path),
			generating_sessions: Arc::new(DashMap::new()),
		}
	}

	/// Tries to acquire generation lock for session_id.
	/// If successful, returns GenerationGuard; if not, returns None.
	pub fn try_acquire_generation(
		&self,
		session_id: Uuid,
	) -> Option<GenerationGuard> {
		// insert returns Option<old_value>, if None — means we inserted first
		if self.generating_sessions.insert(session_id, ()).is_none() {
			Some(GenerationGuard::new(
				session_id,
				self.generating_sessions.clone(),
			))
		} else {
			None
		}
	}

	/// Forcefully releases generation lock (rarely needed, but just in case)
	pub fn force_release_generation(&self, session_id: &Uuid) {
		let _ = self.generating_sessions.remove(session_id);
	}

	fn url(&self) -> String {
		format!(
			"https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent",
			self.location, self.project, self.location, self.model
		)
	}

	/// Returns a Receiver, from which you can read chunks as they arrive.
	pub async fn stream_generate_channel(
		&self,
		req: ChatRequest,
	) -> Result<mpsc::Receiver<Result<String>>> {
		let (tx, rx) = mpsc::channel::<Result<String>>(32);
		let http = self.http.clone();
		let url = self.url();

		let token = match self.access_token.get_token().await {
			Ok(t) => t,
			Err(e) => {
				let _ = tx.send(Err(Error::Token(e))).await;
				return Ok(rx);
			}
		};

		tokio::spawn(async move {
			let mut resp =
				match http.post(&url).bearer_auth(&token).json(&req).send().await {
					Ok(r) => r,
					Err(e) => {
						let _ = tx.send(Err(Error::Reqwest(e))).await;
						return;
					}
				}
				.json_array_stream::<StreamChunk>(usize::MAX);

			while let Some(item) = resp.next().await {
				match item {
					Ok(chunk) => {
						let chunk: StreamChunk = chunk.clone();

						tx.send(Ok(chunk.candidates[0].content.parts[0]
							.text
							.clone()))
							.await
							.unwrap();
					}
					Err(e) => {
						let _ = tx.send(Err(Error::Stream(e))).await;
						return;
					}
				}
			}
		});

		Ok(rx)
	}
}

pub struct GenerationGuard {
	session_id: Uuid,
	map: Arc<DashMap<Uuid, ()>>,
	released: bool,
}

impl GenerationGuard {
	fn new(session_id: Uuid, map: Arc<DashMap<Uuid, ()>>) -> Self {
		Self {
			session_id,
			map,
			released: false,
		}
	}

	/// can be called manually, but Drop also clears
	pub fn release(mut self) {
		self.map.remove(&self.session_id);
		self.released = true;
	}
}

impl Drop for GenerationGuard {
	fn drop(&mut self) {
		if !self.released {
			// ignore result — just remove the key
			let _ = self.map.remove(&self.session_id);
		}
	}
}
