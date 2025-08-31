package com.skyflydev.threadly.data.sessions.dto.response

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class SessionCreatedDto(
    val sessionId: String,

    @SerialName("host_user_id")
    val hostUserId: String,
)