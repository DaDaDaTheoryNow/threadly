package com.skyflydev.threadly.data.sessions.dto.request

import kotlinx.serialization.Serializable

@Serializable
data class CreateSessionRequestDto(
    val theme: String,
    val maxRounds: Int
)