package com.skyflydev.threadly.data.sessions.dto.request

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class SetReadyRequestDto(
    @SerialName("session_id") val sessionId: String,
    @SerialName("is_ready") val isReady: Boolean
)