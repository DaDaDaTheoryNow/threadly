package com.skyflydev.threadly.feature.session.view_model

sealed class SessionIntent {
    data object LeaveSession : SessionIntent()
    data object ToggleReady : SessionIntent()
    data object LoadSessionDetails : SessionIntent()
    data object StartGameAsHost : SessionIntent()
}