package com.skyflydev.threadly.feature.game.view_model

sealed class GameIntent {
    data object ObserveEvents : GameIntent()
    data object StartGame : GameIntent()
    data class SubmitMessage(val content: String) : GameIntent()
}
