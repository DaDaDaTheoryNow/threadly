mod app_state;
mod error;
mod log;
mod mw_auth;
mod mw_res_map;

use axum::{
	Router,
	middleware::{self, from_fn},
};
use lib_auth::router::auth_router;
use lib_core::config::core_config;
use lib_rest::router::rest_router;
use lib_websockets::router::websocket_router;

use crate::app_state::AppState;

#[tokio::main]
pub async fn main() {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter("game_server=debug")
		.init();

	let app_state = AppState::new().await;

	let app = Router::new()
		.merge(auth_router(
			app_state.model_manager.clone(),
			core_config().JWT_SECRET.clone(),
		))
		.merge(
			websocket_router(app_state.game_events_manager.clone())
				.route_layer(from_fn(mw_auth::mw_required_auth)),
		)
		.merge(
			rest_router(
				app_state.model_manager.clone(),
				app_state.game_engine.clone(),
			)
			.route_layer(from_fn(mw_auth::mw_required_auth)),
		)
		.route_layer(from_fn(mw_res_map::mw_response_map))
		.layer(middleware::from_fn_with_state(
			app_state.clone(),
			mw_auth::mw_ctx_resolver,
		));

	println!("Listening on http://localhost:3000");

	// run our app with hyper, listening globally on port 3000
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
