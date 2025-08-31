package com.skyflydev.threadly.feature.auth.view_model

sealed class AuthIntent {
    data class UsernameInput(val value: String) : AuthIntent()
    data class PasswordInput(val value: String) : AuthIntent()

    data class SignIn(val username: String, val password: String) : AuthIntent()
    data class SignUp(val username: String, val password: String) : AuthIntent()
}