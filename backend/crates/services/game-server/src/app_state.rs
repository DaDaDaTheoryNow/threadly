use std::sync::Arc;

use lib_ai::client::AiClient;
use lib_core::{config::core_config, model::ModelManager};
use lib_game_events::manager::GameEventsManager;
use lib_game_logic::engine::GameEngine;

#[derive(Clone)]
pub struct AppState {
	pub model_manager: Arc<ModelManager>,
	pub game_events_manager: Arc<GameEventsManager>,
	pub game_engine: Arc<GameEngine>,
}

impl AppState {
	pub async fn new() -> Self {
		let game_events_manager = Arc::new(GameEventsManager::new());
		let ai_client = Arc::new(AiClient::new(
			&core_config().GCP_PROJECT_ID,
			&core_config().GCP_LOCATION,
			&core_config().GCP_MODEL_NAME,
			&core_config().GCP_APPLICATION_CREDENTIALS,
		));

		let model_manager = Arc::new(ModelManager::new().await.unwrap());

		Self {
			model_manager: model_manager.clone(),
			game_events_manager: game_events_manager.clone(),
			game_engine: Arc::new(GameEngine::new(
				model_manager.clone(),
				game_events_manager.clone(),
				ai_client.clone(),
			)),
		}
	}
}
