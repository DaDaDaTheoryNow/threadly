package com.skyflydev.threadly.feature.home.view_model

import com.skyflydev.threadly.core.model.Session

data class HomeState(
    val sessions: List<Session> = emptyList(),
    val sessionsWithUser: List<Session> = emptyList(),
    val sessionsWithoutUser: List<Session> = emptyList()
)