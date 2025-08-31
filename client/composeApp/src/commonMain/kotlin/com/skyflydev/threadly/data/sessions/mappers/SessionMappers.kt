package com.skyflydev.threadly.data.sessions.mappers

import com.skyflydev.threadly.core.model.Session
import com.skyflydev.threadly.data.sessions.dto.response.SessionDto
import com.skyflydev.threadly.data.sessions.dto.response.SessionEvent

fun SessionDto.toEntity(): Session {
    return Session(
        id = id,
        theme = theme,
        maxRounds = maxRounds,
        playersCount = users.size,
        users = users.map { it.toEntity() }
    )
}

fun SessionEvent.Created.toEntity(): Session {
    return Session(
        id = sessionId,
        theme = theme,
        maxRounds = maxRounds,
        playersCount = users.size,
        users = users.map { it.toEntity() }
    )
}