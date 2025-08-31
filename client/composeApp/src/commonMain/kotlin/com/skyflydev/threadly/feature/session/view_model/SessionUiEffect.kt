package com.skyflydev.threadly.feature.session.view_model

sealed class SessionUiEffect {
    data class NavigateToGameScreen(val sessionId: String) : SessionUiEffect()
    data object NavigateToHomeScreen : SessionUiEffect()

    data object SessionLeave : SessionUiEffect()
    data class ShowMessage(val text: String) : SessionUiEffect()
}