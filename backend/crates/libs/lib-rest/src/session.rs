use axum::{
	Router,
	extract::{Extension, Json, Path},
	http::StatusCode,
	response::IntoResponse,
	routing::{delete, get, post},
};
use lib_core::{ctx::Ctx, model::ModelManager};
use lib_game_logic::engine::GameEngine;
use lib_players::model::Player;
use lib_sessions::model::Session;
use std::sync::Arc;
use uuid::Uuid;

use crate::dto_models::{
	requests::{
		CreateSessionPayload, JoinSessionPayload, LeaveSessionPayload, ReadyPayload,
		StartGamePayload, SubmitMessagePayload,
	},
	responses::{PlayerResponse, SessionResponse, SessionWithUsersDto},
};

use crate::error::Error;

pub fn session_routes() -> Router {
	Router::new()
		.route("/sessions", post(create_session))
		.route("/sessions/join", post(join_session))
		.route("/sessions/leave", delete(leave_session))
		.route("/sessions/ready", post(set_ready))
		.route("/sessions/start", post(start_game))
		.route("/sessions/message", post(submit_message))
		.route("/sessions/{session_id}", get(get_session))
		.route("/sessions", get(get_sessions))
}

async fn create_session(
	ctx: Ctx,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<CreateSessionPayload>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	let session = game_engine.create_session(
		&payload.theme,
		ctx.user_id,
		payload.max_rounds,
	)?;

	let first_player_id = Player::list_by_session(&mut conn, session.id)
		.unwrap()
		.first()
		.unwrap()
		.user_id;

	Ok((
		StatusCode::CREATED,
		Json(SessionResponse {
			session_id: session.id,
			host_user_id: first_player_id,
		}),
	))
}

async fn get_sessions(
	Extension(mm): Extension<Arc<ModelManager>>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	Session::list_with_users(&mut conn)
		.map_err(|e| Error::DbError(e.into()))
		.map(|items| {
			(
				StatusCode::OK,
				Json::<Vec<SessionWithUsersDto>>(
					items.into_iter().map(|item| item.into()).collect(),
				),
			)
		})
}

async fn join_session(
	ctx: Ctx,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<JoinSessionPayload>,
) -> Result<impl IntoResponse, Error> {
	let player = game_engine.join_session(payload.session_id, ctx.user_id)?;

	Ok((
		StatusCode::OK,
		Json(PlayerResponse {
			user_id: player.user_id,
			is_ready: player.is_ready,
			is_host: player.is_host,
		}),
	))
}

async fn leave_session(
	ctx: Ctx,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<LeaveSessionPayload>,
) -> Result<impl IntoResponse, Error> {
	game_engine.leave_session(payload.session_id, ctx.user_id)?;

	Ok(StatusCode::NO_CONTENT.into_response())
}

async fn set_ready(
	ctx: Ctx,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<ReadyPayload>,
) -> Result<impl IntoResponse, Error> {
	let player =
		game_engine.set_ready(payload.session_id, ctx.user_id, payload.is_ready)?;

	Ok((
		StatusCode::OK,
		Json(PlayerResponse {
			user_id: player.user_id,
			is_ready: player.is_ready,
			is_host: player.is_host,
		}),
	))
}

async fn start_game(
	ctx: Ctx,
	Extension(mm): Extension<Arc<ModelManager>>,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<StartGamePayload>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	let session = game_engine.start_game(payload.session_id, ctx.user_id)?;

	let first_player_id = Player::list_by_session(&mut conn, session.id)
		.unwrap()
		.first()
		.unwrap()
		.user_id;

	Ok((
		StatusCode::OK,
		Json(SessionResponse {
			session_id: session.id,
			host_user_id: first_player_id,
		}),
	))
}

async fn submit_message(
	ctx: Ctx,
	Extension(game_engine): Extension<Arc<GameEngine>>,
	Json(payload): Json<SubmitMessagePayload>,
) -> Result<impl IntoResponse, Error> {
	let _ = game_engine.submit_message(
		payload.session_id,
		ctx.user_id,
		&payload.content,
	)?;

	Ok(StatusCode::OK.into_response())
}

async fn get_session(
	Path(session_id): Path<Uuid>,
	Extension(mm): Extension<Arc<ModelManager>>,
) -> Result<impl IntoResponse, Error> {
	let mut conn = mm.db();

	match Session::get_with_users(&mut conn, session_id)
		.map_err(|e| Error::DbError(e.into()))?
	{
		Some(item) => Ok((StatusCode::OK, Json::<SessionWithUsersDto>(item.into()))
			.into_response()),
		None => Ok(StatusCode::NOT_FOUND.into_response()),
	}
}
