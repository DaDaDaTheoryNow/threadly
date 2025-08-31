package com.skyflydev.threadly.core.model

import kotlinx.serialization.Serializable

@Serializable
data class Player(
    val userId: String,
    val isReady: Boolean,
    val isHost: Boolean
)