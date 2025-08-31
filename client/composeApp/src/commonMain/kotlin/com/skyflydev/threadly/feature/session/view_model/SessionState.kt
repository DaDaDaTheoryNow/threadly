package com.skyflydev.threadly.feature.session.view_model

import com.skyflydev.threadly.core.model.Session

data class SessionState(
    val currentSession: Session? = null,
    val isUserReady: Boolean = false,
    val isLoading: Boolean = false,
    val isHost: Boolean = false
)