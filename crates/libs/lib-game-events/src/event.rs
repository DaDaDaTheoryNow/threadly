use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameEvent {
	GameStarted { session_id: Uuid },
	NewTurn { player_id: Uuid },
	PlayerLeft { player_id: Uuid },
	GameFinished { session_id: Uuid },
	PlayerJoined { player_id: Uuid },
	PlayerReady { player_id: Uuid, ready: bool },
	LastPlayerMessage { content: String },
	Error { message: String },
}

pub struct GameEventReceiver {
	pub player_id: Uuid,
}
