use std::sync::Arc;

use crate::session::session_routes;
use axum::Router;
use lib_core::model::ModelManager;
use lib_game_logic::engine::GameEngine;

pub fn rest_router(mm: Arc<ModelManager>, game_engine: Arc<GameEngine>) -> Router {
	Router::new()
		.nest("/api", session_routes())
		.layer(axum::Extension(mm))
		.layer(axum::Extension(game_engine))
}
