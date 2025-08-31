use dashmap::DashMap;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::event::{
	game::{GameEvent, GameEventReceiver},
	session::SessionEvent,
};

#[derive(Clone)]
pub struct GameEventsManager {
	/// player_senders: session_id -> (user_id -> sender)
	player_senders: DashMap<Uuid, DashMap<Uuid, broadcast::Sender<GameEvent>>>,

	/// session_senders: user_id -> sender for session-level observers
	session_senders: DashMap<Uuid, broadcast::Sender<SessionEvent>>,
}

impl GameEventsManager {
	pub fn new() -> Self {
		Self {
			player_senders: DashMap::new(),
			session_senders: DashMap::new(),
		}
	}

	// --------------------
	// Subscriptions
	// --------------------

	/// Subscribe as user — receive `GameEvent` for specific player in specific session.
	pub fn subscribe_user_to_observe_game_events(
		&self,
		session_id: Uuid,
		user_id: Uuid,
	) -> broadcast::Receiver<GameEvent> {
		let player_map = self
			.player_senders
			.entry(session_id)
			.or_insert_with(DashMap::new);

		let sender = player_map
			.entry(user_id)
			.or_insert_with(|| broadcast::channel(100).0);
		sender.subscribe()
	}

	/// Subscribe as session observer — receive `SessionEvent`.
	pub fn subscribe_user_to_observe_sessions_list(
		&self,
		user_id: Uuid,
	) -> broadcast::Receiver<SessionEvent> {
		let sender = self
			.session_senders
			.entry(user_id)
			.or_insert_with(|| broadcast::channel(100).0);
		sender.subscribe()
	}

	/// Send `GameEvent` either to all players in session, or to a specific player (if receiver is specified).
	pub fn send_game_event(
		&self,
		session_id: Uuid,
		game_event_receiver: Option<GameEventReceiver>,
		event: GameEvent,
	) {
		match game_event_receiver {
			Some(receiver) => {
				if let Some(player_map) = self.player_senders.get(&session_id) {
					if let Some(sender) = player_map.get(&receiver.user_id) {
						let _ = sender.send(event);
					}
				}
			}
			None => {
				// send to all per-player senders
				if let Some(player_map) = self.player_senders.get(&session_id) {
					for sender in player_map.iter() {
						let _ = sender.value().send(event.clone());
					}
				}
			}
		}
	}

	/// Send `SessionEvent` to all session observers.
	pub fn send_session_event(&self, event: SessionEvent) {
		for sender in self.session_senders.iter() {
			let _ = sender.send(event.clone());
		}
	}
}
