use axum::{
	Router,
	extract::{Extension, Json, Path},
	http::StatusCode,
	response::IntoResponse,
	routing::{delete, get, post},
};
use lib_core::{
	ctx::Ctx,
	model::{ModelManager, base::BasicDbOps},
};
use lib_game_logic::engine::GameEngine;
use lib_players::model::Player;
use lib_sessions::model::Session;
use std::sync::Arc;
use uuid::Uuid;

use crate::dto_models::{
	CreateSessionPayload, PlayerResponse, ReadyPayload, SessionResponse,
	StartGamePayload, SubmitMessagePayload,
};
use crate::error::Error;

pub fn session_routes() -> Router {
	Router::new()
		.route("/sessions", post(create_session))
		.route("/sessions/{session_id}/join", post(join_session))
		.route(
			"/sessions/{session_id}/leave/{player_id}",
			delete(leave_session),
		)
		.route("/sessions/{session_id}/ready", post(set_ready))
		.route("/sessions/{session_id}/start", post(start_game))
		.route("/sessions/{session_id}/message", post(submit_message))
		.route("/sessions/{session_id}", get(get_session))
}

async fn create_session(
	ctx: Ctx,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<CreateSessionPayload>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	let session = game_engine.create_session(
		&mut conn,
		&payload.theme,
		ctx.user_id,
		payload.max_rounds,
	)?;

	let first_player_id = Player::list_by_session(&mut conn, session.id)
		.unwrap()
		.first()
		.unwrap()
		.id;

	Ok((
		StatusCode::CREATED,
		Json(SessionResponse {
			session_id: session.id,
			player_id: first_player_id,
		}),
	))
}

async fn join_session(
	Path(session_id): Path<Uuid>,
	ctx: Ctx,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	let player = game_engine.join_session(&mut conn, session_id, ctx.user_id)?;

	Ok((
		StatusCode::OK,
		Json(PlayerResponse {
			player_id: player.id,
			is_ready: player.is_ready,
			is_host: player.is_host,
		}),
	))
}

async fn leave_session(
	Path((session_id, player_id)): Path<(Uuid, Uuid)>,
	ctx: Ctx,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	game_engine.leave_session(&mut conn, session_id, player_id, ctx.user_id)?;

	Ok(StatusCode::NO_CONTENT.into_response())
}

async fn set_ready(
	ctx: Ctx,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<ReadyPayload>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	let player = game_engine.set_ready(
		&mut conn,
		payload.player_id,
		ctx.user_id,
		payload.ready,
	)?;

	Ok((
		StatusCode::OK,
		Json(PlayerResponse {
			player_id: player.id,
			is_ready: player.is_ready,
			is_host: player.is_host,
		}),
	))
}

async fn start_game(
	ctx: Ctx,
	Path(session_id): Path<Uuid>,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<StartGamePayload>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	let session = game_engine.start_game(
		&mut conn,
		session_id,
		payload.host_player_id,
		ctx.user_id,
	)?;

	let first_player_id = Player::list_by_session(&mut conn, session.id)
		.unwrap()
		.first()
		.unwrap()
		.id;

	Ok((
		StatusCode::OK,
		Json(SessionResponse {
			session_id: session.id,
			player_id: first_player_id,
		}),
	))
}

async fn submit_message(
	Path(session_id): Path<Uuid>,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<SubmitMessagePayload>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	let _ = game_engine.submit_message(
		&mut conn,
		session_id,
		payload.player_id,
		&payload.content,
	)?;

	Ok(StatusCode::OK.into_response())
}

async fn get_session(
	Path(session_id): Path<Uuid>,
	Extension(mm): Extension<Arc<ModelManager>>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	Session::get(&mut conn, session_id)
		.map_err(|e| Error::DbError(e.into()))
		.map(|session| (StatusCode::OK, Json(session)))
}
