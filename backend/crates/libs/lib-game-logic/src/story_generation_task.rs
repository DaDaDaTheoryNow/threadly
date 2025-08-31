use std::sync::Arc;

use lib_ai::{
	client::{AiClient, GenerationGuard},
	models::{ChatRequest, Content, Part, SystemInstruction},
};
use lib_core::model::{ModelManager, base::BasicDbOps, schema_enums::SessionStatus};
use lib_game_events::{event::game::GameEvent, manager::GameEventsManager};
use lib_messages::model::Message;
use lib_sessions::model::Session;
use lib_stories::model::{NewStory, Story};
use uuid::Uuid;

/// Spawn an async task that collects messages, streams generation via AI client,
/// relays chunks to events, saves the story and finalizes the session.
pub fn spawn_story_generation_task(
	session_id: Uuid,
	generation_guard: GenerationGuard, // use whatever concrete guard type your ai_client returns
	model_manager: Arc<ModelManager>,
	ai_client: Arc<AiClient>,
	events: Arc<GameEventsManager>,
) {
	// Clone everything needed into the task
	tokio::spawn(async move {
		// Ensure the guard is held inside the task's scope
		let _guard = generation_guard;

		let model_manager_clone = model_manager.clone();

		// Step A: build prompt from messages in blocking thread
		let prompt = tokio::task::spawn_blocking(move || {
			let mut conn = model_manager.db();
			match Message::list_by_session(&mut conn, session_id) {
				Ok(msgs) => {
					let mut p = String::from("User messages::\n");
					for m in msgs {
						p.push_str(&format!("- {}\n", m.content));
					}
					p
				}
				Err(e) => {
					eprintln!("Failed to load messages for generation: {:?}", e);
					String::new()
				}
			}
		})
		.await
		.unwrap_or_default();

		// Build chat request
		let chat_req = ChatRequest {
                system_instruction: Some(SystemInstruction {
                    parts: vec![Part {
                        text: "You are a storyteller... Collect all player messages into a story.".into(),
                    }],
                }),
                contents: vec![Content {
                    role: "user".into(),
                    parts: vec![Part { text: prompt }],
                }],
            };

		// Step B: stream generation from ai_client
		match ai_client.stream_generate_channel(chat_req).await {
			Ok(mut rx) => {
				let mut seq = 0u64;
				let mut full = String::new();

				while let Some(item) = rx.recv().await {
					match item {
						Ok(chunk) => {
							seq += 1;
							full.push_str(&chunk);
							// send chunk event to all clients
							events.send_game_event(
								session_id,
								None,
								GameEvent::StoryChunk { seq, chunk },
							);
						}
						Err(e) => {
							events.send_game_event(
								session_id,
								None,
								GameEvent::StoryComplete {
									story_id: Uuid::new_v4(),
									full_text: format!("Generation error: {:?}", e),
								},
							);
							return;
						}
					}
				}

				// Step C: persist full story in blocking thread
				let full_clone = full.clone();
				let mm_for_save = model_manager_clone.clone();
				let events_for_finish = events.clone();
				tokio::task::spawn_blocking(move || {
					let mut conn = mm_for_save.db();
					let new_story = NewStory {
						session_id,
						content: &full_clone,
					};

					match Story::create(&mut conn, new_story) {
						Ok(story) => {
							// notify clients about final story
							events_for_finish.send_game_event(
								session_id,
								None,
								GameEvent::StoryComplete {
									story_id: story.id,
									full_text: full_clone.clone(),
								},
							);

							// finish session in DB
							let mut conn2 = mm_for_save.db();
							if let Ok(mut session) =
								Session::get(&mut conn2, session_id)
							{
								session.status = SessionStatus::Finished;
								session.current_user_id_turn = None;
								let _ = Session::update(
									&mut conn2, session.id, &session,
								);
							}

							// and notify that game finished
							events_for_finish.send_game_event(
								session_id,
								None,
								GameEvent::GameFinished,
							);
						}
						Err(e) => {
							eprintln!("Failed to save story: {:?}", e);
							events_for_finish.send_game_event(
								session_id,
								None,
								GameEvent::StoryComplete {
									story_id: Uuid::new_v4(),
									full_text: format!(
										"Failed to save story: {:?}",
										e
									),
								},
							);
						}
					}
				});
			}
			Err(e) => {
				events.send_game_event(
					session_id,
					None,
					GameEvent::StoryComplete {
						story_id: Uuid::new_v4(),
						full_text: format!("Failed to start generation: {:?}", e),
					},
				);
			}
		}
	});
}
