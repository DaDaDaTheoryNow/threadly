use std::sync::Arc;

use lib_core::model::ModelManager;
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

		Self {
			model_manager: Arc::new(ModelManager::new().await.unwrap()),
			game_events_manager: game_events_manager.clone(),
			game_engine: Arc::new(GameEngine::new(game_events_manager.clone())),
		}
	}
}
