package com.skyflydev.threadly.data.sessions.mappers

import com.skyflydev.threadly.core.model.Player
import com.skyflydev.threadly.data.sessions.dto.response.SessionPlayerDto

fun SessionPlayerDto.toEntity(): Player {
    return Player(
        userId = this.userId,
        isReady = this.isReady,
        isHost = this.isHost
    )
}