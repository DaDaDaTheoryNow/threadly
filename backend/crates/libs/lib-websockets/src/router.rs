use std::sync::Arc;

use crate::{
	game_events::handle_game_events_socket,
	sessions_events::handle_sessions_events_socket,
};
use axum::{
	Extension, Router,
	extract::{Path, ws::WebSocketUpgrade},
	response::IntoResponse,
	routing::get,
};
use lib_core::ctx::Ctx;
use lib_game_events::manager::GameEventsManager;
use uuid::Uuid;

pub async fn ws_observe_game_events_handler(
	ctx: Ctx,
	Path(session_id): Path<Uuid>,
	Extension(game_events_manager): Extension<Arc<GameEventsManager>>,
	ws: WebSocketUpgrade,
) -> impl IntoResponse {
	ws.on_upgrade(move |socket| {
		handle_game_events_socket(
			socket,
			session_id,
			game_events_manager,
			ctx.user_id,
		)
	})
}

pub async fn ws_observe_sessions_events_handler(
	_ctx: Ctx,
	Extension(game_events_manager): Extension<Arc<GameEventsManager>>,
	ws: WebSocketUpgrade,
) -> impl IntoResponse {
	ws.on_upgrade(move |socket| {
		handle_sessions_events_socket(socket, Uuid::new_v4(), game_events_manager)
	})
}

pub fn websocket_router(game_events_manager: Arc<GameEventsManager>) -> Router {
	Router::new()
		.route(
			"/observe/game/{session_id}",
			get(ws_observe_game_events_handler),
		)
		.route("/observe/sessions", get(ws_observe_sessions_events_handler))
		.layer(Extension(game_events_manager))
}
