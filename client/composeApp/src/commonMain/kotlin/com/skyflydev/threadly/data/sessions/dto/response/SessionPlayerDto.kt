package com.skyflydev.threadly.data.sessions.dto.response

import kotlinx.serialization.Serializable

@Serializable
data class SessionPlayerDto(
    val userId: String,
    val isReady: Boolean,
    val isHost: Boolean
)
