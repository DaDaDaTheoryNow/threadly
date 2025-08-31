package com.skyflydev.threadly.core.model

sealed class DomainGameEvent {
    object GameStarted : DomainGameEvent()
    data class NewTurn(val userId: String) : DomainGameEvent()
    data class PlayerLeft(val userId: String) : DomainGameEvent()
    object GameFinished : DomainGameEvent()
    data class PlayerJoined(val userId: String) : DomainGameEvent()
    data class PlayerReady(val userId: String, val ready: Boolean) : DomainGameEvent()
    data class LastPlayerMessage(val content: String) : DomainGameEvent()
    data class Error(val message: String) : DomainGameEvent()
    object SessionDeleted : DomainGameEvent()
    object WaitingForStoryGeneration : DomainGameEvent()
    data class StoryChunk(val seq: Long, val chunk: String) : DomainGameEvent()
    data class StoryComplete(val storyId: String, val fullText: String) : DomainGameEvent()
}
