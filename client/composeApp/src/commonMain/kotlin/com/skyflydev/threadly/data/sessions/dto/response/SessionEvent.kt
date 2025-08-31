package com.skyflydev.threadly.data.sessions.dto.response

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
sealed class SessionEvent {

    @Serializable
    @SerialName("created")
    data class Created(
        @SerialName("session_id")
        val sessionId: String,
        @SerialName("theme")
        val theme: String,
        @SerialName("max_rounds")
        val maxRounds: Int,
        @SerialName("users")
        val users: List<SessionPlayerDto>
    ) : SessionEvent()

    @Serializable
    @SerialName("update_players")
    data class UpdatePlayers(
        @SerialName("session_id")
        val sessionId: String,
        @SerialName("users")
        val users: List<SessionPlayerDto>
    ) : SessionEvent()

    @Serializable
    @SerialName("started")
    data class Started(
        @SerialName("session_id")
        val sessionId: String
    ) : SessionEvent()

    @Serializable
    @SerialName("deleted")
    data class Deleted(
        @SerialName("session_id")
        val sessionId: String
    ) : SessionEvent()
}
