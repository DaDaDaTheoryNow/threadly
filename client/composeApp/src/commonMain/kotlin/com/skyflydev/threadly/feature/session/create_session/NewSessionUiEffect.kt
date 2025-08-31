package com.skyflydev.threadly.feature.session.create_session

sealed class NewSessionUiEffect {
    data class NavigateToSessionScreen(val sessionId: String, val hostUserId: String): NewSessionUiEffect()
}