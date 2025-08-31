package com.skyflydev.threadly.feature.home.view_model

sealed class HomeIntent {
    data class JoinSession(val sessionId: String) : HomeIntent()
    data class LeaveSession(val sessionId: String) : HomeIntent()
    data object LoadSessions : HomeIntent()
    data object ObserveSessionEvents : HomeIntent()

    data object SignOut : HomeIntent()
}
