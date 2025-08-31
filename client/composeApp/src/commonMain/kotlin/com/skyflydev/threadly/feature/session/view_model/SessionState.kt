package com.skyflydev.threadly.feature.session.view_model

import com.skyflydev.threadly.core.model.Session

data class SessionState(
    val currentSession: Session? = null,
    val isUserReady: Boolean = false,
    val isLoading: Boolean = false,

    // gameplay
    val isHost: Boolean = false,
    val players: List<String> = emptyList(),

    val isGameStarted: Boolean = false,
    val currentTurnUserId: String? = null,
    val lastPlayerMessage: String? = null,
    val isMyTurn: Boolean = false,

    val isWaitingForStoryGeneration: Boolean = false,
    val storyChunks: List<Pair<Long, String>> = emptyList(), // (seq, chunk)
    val storySoFar: String = "", // incremental text assembled from chunks
    val storyFullText: String? = null,

    val isGameFinished: Boolean = false,
)