package com.skyflydev.threadly.feature.game.view_model

data class GameState(
    val players: List<String> = emptyList(),
    val isGameStarted: Boolean = false,
    val currentTurnUserId: String? = null,
    val lastPlayerMessage: String? = null,
    val isMyTurn: Boolean = false,
    val isWaitingForStoryGeneration: Boolean = false,
    val storyChunks: List<Pair<Long, String>> = emptyList(),
    val storySoFar: String = "",
    val storyFullText: String? = null,
    val isGameFinished: Boolean = false,
    val isHost: Boolean = false
)
