package com.skyflydev.threadly.feature.session.create

sealed class NewSessionUiEffect {
    data class NavigateToSessionScreen(val sessionId: String, val userId: String): NewSessionUiEffect()
}