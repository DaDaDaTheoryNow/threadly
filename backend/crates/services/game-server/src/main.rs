mod app_state;
mod error;
mod log;
mod mw_auth;
mod mw_res_map;

use std::time::Duration;

use axum::{
	Router,
	http::{
		HeaderValue, Method,
		header::{AUTHORIZATION, CONTENT_TYPE},
	},
	middleware::{self, from_fn},
};
use lib_auth::router::auth_router;
use lib_core::config::core_config;
use lib_rest::router::rest_router;
use lib_websockets::router::websocket_router;
use tower_http::cors::CorsLayer;

use crate::app_state::AppState;

#[tokio::main]
pub async fn main() {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter("game_server=debug")
		.init();

	let app_state = AppState::new().await;

	// let mut conn = app_state.model_manager.db();
	// let session_id = Session::list(&mut conn).unwrap().first().unwrap().id;

	// if let Some(generation_guard) = app_state
	// 	.game_engine
	// 	.ai_client
	// 	.try_acquire_generation(session_id)
	// {
	// 	let cloned_model_manager = app_state.model_manager.clone();
	// 	let cloned_model_manager_save_full_story = app_state.model_manager.clone();

	// 	let cloned_ai_client = app_state.game_engine.ai_client.clone();
	// 	let cloned_events = app_state.game_events_manager.clone();

	// 	tokio::spawn(async move {
	// 		let _guard = generation_guard;

	// 		let prompt_res: String = tokio::task::spawn_blocking(move || {
	// 			let mut conn = cloned_model_manager.db();

	// 			let msgs = match Message::list_by_session(&mut conn, session_id) {
	// 				Ok(msgs) => msgs,
	// 				Err(_) => return String::new(),
	// 			};

	// 			let mut p = "Собери ходы игроков в связную историю::\n".to_string();
	// 			for m in msgs {
	// 				p.push_str(&format!("- {}\n", m.content));
	// 			}

	// 			p
	// 		})
	// 		.await
	// 		.unwrap_or_else(|e| {
	// 			eprintln!("spawn_blocking join err: {:?}", e);
	// 			String::new()
	// 		});

	// 		println!("prompt_res: {}", prompt_res);

	// 		let chat_req = ChatRequest {
	// 			system_instruction: Some(SystemInstruction {
	// 				parts: vec![Part {
	// 					text: "You are a storyteller...".into(),
	// 				}],
	// 			}),
	// 			contents: vec![Content {
	// 				role: "user".into(),
	// 				parts: vec![Part { text: prompt_res }],
	// 			}],
	// 		};

	// 		match cloned_ai_client.stream_generate_channel(chat_req).await {
	// 			Ok(mut rx) => {
	// 				let mut full = String::new();
	// 				while let Some(item) = rx.recv().await {
	// 					match item {
	// 						Ok(chunk) if chunk == "[DONE]" => break,
	// 						Ok(chunk) => {
	// 							full.push_str(&chunk);

	// 							eprintln!("Generated chunk: {}", chunk);
	// 							// cloned_events.send(
	// 							// 	session_id,
	// 							// 	None,
	// 							// 	GameEvent::StoryChunk { seq, chunk },
	// 							// );
	// 						}
	// 						Err(e) => {
	// 							eprintln!("Generation error: {:?}", e);
	// 							// cloned_events.send(
	// 							// 	session_id,
	// 							// 	None,
	// 							// 	GameEvent::StoryComplete {
	// 							// 		story_id: uuid::Uuid::new_v4(),
	// 							// 		full_text: format!(
	// 							// 			"Generation error: {:?}",
	// 							// 			e
	// 							// 		),
	// 							// 	},
	// 							// );
	// 							return;
	// 						}
	// 					}
	// 				}

	// 				println!("Full story: {}", full);
	// 			}
	// 			Err(e) => {
	// 				println!("Failed to start generation: {:?}", e);
	// 			}
	// 		}
	// 	});
	// }

	let allowed_origins = vec![
		HeaderValue::from_static("http://localhost:8080"),
		HeaderValue::from_static("http://192.168.100.11:8080"),
		HeaderValue::from_static("http://192.168.0.101:8080"),
	];

	let cors = CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([Method::GET, Method::POST, Method::DELETE])
		.allow_headers([CONTENT_TYPE, AUTHORIZATION])
		.allow_credentials(true)
		.max_age(Duration::from_secs(3600));

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
		))
		.layer(cors);

	println!("Listening on http://localhost:3000");

	// run our app with hyper, listening globally on port 3000
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
