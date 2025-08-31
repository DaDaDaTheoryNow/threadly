use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameEvent {
	GameStarted,
	NewTurn { user_id: Uuid },
	PlayerLeft { user_id: Uuid },
	GameFinished,
	PlayerJoined { user_id: Uuid },
	PlayerReady { user_id: Uuid, ready: bool },
	LastPlayerMessage { content: String },
	Error { message: String },
	SessionDeleted,

	// Events that depend on ai
	WaitingForStoryGeneration,
	StoryChunk { seq: u64, chunk: String },
	StoryComplete { story_id: Uuid, full_text: String },
}

pub struct GameEventReceiver {
	pub user_id: Uuid,
}
