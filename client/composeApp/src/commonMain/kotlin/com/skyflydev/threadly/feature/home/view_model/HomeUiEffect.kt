package com.skyflydev.threadly.feature.home.view_model

sealed class HomeUiEffect {
    data class NavigateToSession(val sessionId: String, val userId: String) : HomeUiEffect()
    data class ShowMessage(val text: String) : HomeUiEffect()
}