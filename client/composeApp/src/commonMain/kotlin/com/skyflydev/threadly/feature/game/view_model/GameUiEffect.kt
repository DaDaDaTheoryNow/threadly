package com.skyflydev.threadly.feature.game.view_model

sealed class GameUiEffect {
    data class ShowMessage(val text: String) : GameUiEffect()
}
