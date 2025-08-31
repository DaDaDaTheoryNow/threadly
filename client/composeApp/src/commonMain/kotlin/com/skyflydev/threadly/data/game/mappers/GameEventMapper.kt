package com.skyflydev.threadly.data.game.mappers

import com.skyflydev.threadly.core.model.DomainGameEvent
import com.skyflydev.threadly.data.game.dto.GameEvent

fun GameEvent.toDomain(): DomainGameEvent {
    return when (this) {
        is GameEvent.Error -> DomainGameEvent.Error(message)
        GameEvent.GameFinished -> DomainGameEvent.GameFinished
        GameEvent.GameStarted -> DomainGameEvent.GameStarted
        is GameEvent.LastPlayerMessage -> DomainGameEvent.LastPlayerMessage(content)
        is GameEvent.NewTurn -> DomainGameEvent.NewTurn(userId)
        is GameEvent.PlayerJoined -> DomainGameEvent.PlayerJoined(userId)
        is GameEvent.PlayerLeft -> DomainGameEvent.PlayerLeft(userId)
        is GameEvent.PlayerReady -> DomainGameEvent.PlayerReady(userId, ready)
        GameEvent.SessionDeleted -> DomainGameEvent.SessionDeleted
        is GameEvent.StoryChunk -> DomainGameEvent.StoryChunk(seq, chunk)
        is GameEvent.StoryComplete -> DomainGameEvent.StoryComplete(storyId, fullText)
        GameEvent.WaitingForStoryGeneration -> DomainGameEvent.WaitingForStoryGeneration
    }
}
