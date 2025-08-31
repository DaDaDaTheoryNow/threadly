package com.skyflydev.threadly.feature.auth.view_model

data class AuthState(
    val username: String = "",
    val password: String = "",
    val isLoading: Boolean = false,
    val errorMessage: String? = null
)