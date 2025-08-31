package com.skyflydev.threadly.data.game.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
sealed class GameEvent {

    @Serializable
    @SerialName("game_started")
    object GameStarted : GameEvent()

    @Serializable
    @SerialName("new_turn")
    data class NewTurn(
        @SerialName("user_id")
        val userId: String
    ) : GameEvent()

    @Serializable
    @SerialName("player_left")
    data class PlayerLeft(
        @SerialName("user_id")
        val userId: String
    ) : GameEvent()

    @Serializable
    @SerialName("game_finished")
    object GameFinished : GameEvent()

    @Serializable
    @SerialName("player_joined")
    data class PlayerJoined(
        @SerialName("user_id")
        val userId: String
    ) : GameEvent()

    @Serializable
    @SerialName("player_ready")
    data class PlayerReady(
        @SerialName("user_id")
        val userId: String,
        @SerialName("ready")
        val ready: Boolean
    ) : GameEvent()

    @Serializable
    @SerialName("last_player_message")
    data class LastPlayerMessage(
        @SerialName("content")
        val content: String
    ) : GameEvent()

    @Serializable
    @SerialName("error")
    data class Error(
        @SerialName("message")
        val message: String
    ) : GameEvent()

    @Serializable
    @SerialName("session_deleted")
    object SessionDeleted : GameEvent()

    // Events that depend on AI
    @Serializable
    @SerialName("waiting_for_story_generation")
    object WaitingForStoryGeneration : GameEvent()

    @Serializable
    @SerialName("story_chunk")
    data class StoryChunk(
        @SerialName("seq")
        val seq: Long,
        @SerialName("chunk")
        val chunk: String
    ) : GameEvent()

    @Serializable
    @SerialName("story_complete")
    data class StoryComplete(
        @SerialName("story_id")
        val storyId: String,
        @SerialName("full_text")
        val fullText: String
    ) : GameEvent()
}
