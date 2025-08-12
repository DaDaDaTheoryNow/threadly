use dashmap::DashMap;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::event::{GameEvent, GameEventReceiver};

#[derive(Clone)]
pub struct GameEventsManager {
	senders: DashMap<Uuid, DashMap<Uuid, broadcast::Sender<GameEvent>>>,
}

impl GameEventsManager {
	pub fn new() -> Self {
		Self {
			senders: DashMap::new(),
		}
	}

	pub fn subscribe(
		&self,
		session_id: Uuid,
		player_id: Uuid,
	) -> broadcast::Receiver<GameEvent> {
		let player_map = self.senders.entry(session_id).or_insert_with(DashMap::new);
		let sender = player_map
			.entry(player_id)
			.or_insert_with(|| broadcast::channel(100).0);
		sender.subscribe()
	}

	fn send_to_all(&self, session_id: Uuid, event: GameEvent) {
		if let Some(player_map) = self.senders.get(&session_id) {
			for sender in player_map.iter() {
				let _ = sender.value().send(event.clone());
			}
		}
	}

	fn send_to_player(&self, session_id: Uuid, player_id: Uuid, event: GameEvent) {
		if let Some(player_map) = self.senders.get(&session_id) {
			if let Some(sender) = player_map.get(&player_id) {
				let _ = sender.send(event);
			}
		}
	}

	pub fn send(
		&self,
		session_id: Uuid,
		game_event_receiver: Option<GameEventReceiver>,
		event: GameEvent,
	) {
		match game_event_receiver {
			Some(receiver) => {
				self.send_to_player(session_id, receiver.player_id, event);
			}
			None => {
				self.send_to_all(session_id, event);
			}
		}
	}
}
