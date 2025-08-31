package com.skyflydev.threadly.data.auth.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class AuthResponseDto(
    val token: String,
    @SerialName("user_id")
    val userId: String
)