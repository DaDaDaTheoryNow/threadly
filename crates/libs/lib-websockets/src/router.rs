use std::sync::Arc;

use crate::connection::handle_socket;
use axum::{
	Extension, Router,
	extract::{Path, ws::WebSocketUpgrade},
	response::IntoResponse,
	routing::get,
};
use lib_game_events::manager::GameEventsManager;
use uuid::Uuid;

pub async fn ws_handler(
	Path((session_id, player_id)): Path<(Uuid, Uuid)>,
	Extension(game_events_manager): Extension<Arc<GameEventsManager>>,
	ws: WebSocketUpgrade,
) -> impl IntoResponse {
	ws.on_upgrade(move |socket| {
		handle_socket(socket, session_id, game_events_manager, player_id)
	})
}

pub fn websocket_router(game_events_manager: Arc<GameEventsManager>) -> Router {
	Router::new()
		.route("/{session_id}/{player_id}", get(ws_handler))
		.layer(Extension(game_events_manager))
}
