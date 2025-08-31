use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use lib_game_events::{event::session::SessionEvent, manager::GameEventsManager};
use std::sync::Arc;
use uuid::Uuid;

use tokio::select;

pub async fn handle_sessions_events_socket(
	socket: WebSocket,
	user_id: Uuid,
	game_events_manager: Arc<GameEventsManager>,
) {
	let mut receiver =
		game_events_manager.subscribe_user_to_observe_sessions_list(user_id);

	let (mut ws_sender, mut ws_receiver) = socket.split();

	let mut write_task = tokio::spawn(async move {
		while let Ok(msg) = receiver.recv().await {
			if let Err(e) = send_msg(&mut ws_sender, msg).await {
				eprintln!("Failed to send message to websocket: {:?}", e);
				break;
			}
		}
	});

	loop {
		select! {
			ws_msg = ws_receiver.next() => {
				match ws_msg {
					Some(Ok(msg)) => {
						if msg == Message::Close(None) {
							break;
						}
					}
					_ => {
						break;
					}
				}
			}
			_ = &mut write_task => {
				break;
			}
		}
	}

	write_task.abort();
}

async fn send_msg(
	socket: &mut SplitSink<WebSocket, axum::extract::ws::Message>,
	msg: SessionEvent,
) -> Result<(), axum::Error> {
	if let Ok(json) = serde_json::to_string(&msg) {
		socket.send(Message::Text(json.into())).await?;
	}
	Ok(())
}
