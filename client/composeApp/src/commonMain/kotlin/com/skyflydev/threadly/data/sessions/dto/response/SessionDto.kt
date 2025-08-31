package com.skyflydev.threadly.data.sessions.dto.response

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class SessionDto(
    val id: String,
    val theme: String,

    @SerialName("max_rounds")
    val maxRounds: Int,

    @SerialName("current_round")
    val currentRound: Int,

    @SerialName("users")
    val users: List<SessionPlayerDto>,
)