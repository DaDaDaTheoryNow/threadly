package com.skyflydev.threadly.core.model

import kotlinx.serialization.Serializable

@Serializable
data class Session(
    val id: String,
    val theme: String,
    val maxRounds: Int,
    val playersCount: Int,
    val users: List<Player>,
)