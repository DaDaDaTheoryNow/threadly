package com.skyflydev.threadly.feature.game.view_model

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.skyflydev.threadly.core.common.error.toMessage
import com.skyflydev.threadly.core.common.result.onError
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.core.model.DomainGameEvent
import com.skyflydev.threadly.data.game.repository.GameRepository
import com.skyflydev.threadly.feature.game.view_model.GameUiEffect.ShowMessage
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class GameViewModel(
    private val sessionId: String,
    private val userId: String,
    private val isHost: Boolean,
    private val gameRepository: GameRepository
) : ViewModel() {
    private val _state = MutableStateFlow(GameState())
    val state = _state.asStateFlow()

    private val _effects = MutableSharedFlow<GameUiEffect>(extraBufferCapacity = 8)
    val effects = _effects.asSharedFlow()

    init {
        if (isHost) {
            sendIntent(GameIntent.StartGame)
        }
        sendIntent(GameIntent.ObserveEvents)
    }

    fun sendIntent(intent: GameIntent) {
        when (intent) {
            is GameIntent.SubmitMessage -> handleSubmitMessage(intent)
            is GameIntent.StartGame -> handleStartGame()
            GameIntent.ObserveEvents -> observeGameEvents()
        }
    }

    private fun handleStartGame() {
        viewModelScope.launch {
            gameRepository.startGame(sessionId)
                .onError { error ->
                    _effects.emit(ShowMessage(error.toMessage()))
                }
        }
    }

    private fun observeGameEvents() {
        gameRepository.gameEventsFlow(sessionId)
            .onEach { event ->
                println("GameEvent: $event")
                reduceGameEvent(event)
            }
            .launchIn(viewModelScope)
    }

    private fun handleSubmitMessage(intent: GameIntent.SubmitMessage) {
        viewModelScope.launch {
            gameRepository.submitMessage(sessionId, intent.content)
                .onSuccess {
                    reduce { copy(lastPlayerMessage = null, isMyTurn = false) }
                }
                .onError { error ->
                    _effects.emit(ShowMessage(error.toMessage()))
                }
        }
    }

    private fun reduce(reducer: GameState.() -> GameState) {
        _state.update { it.reducer() }
    }

    private fun reduceGameEvent(event: DomainGameEvent) {
        when (event) {
            is DomainGameEvent.Error -> {
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Server error: ${event.message}"))
                }
            }
            DomainGameEvent.GameFinished -> {
                reduce { copy(isGameFinished = true) }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Game finished"))
                }
            }
            is DomainGameEvent.LastPlayerMessage -> {
                reduce {
                    copy(lastPlayerMessage = event.content)
                }
            }
            is DomainGameEvent.NewTurn -> {
                val isMine = event.userId == userId
                reduce {
                    copy(
                        currentTurnUserId = event.userId,
                        isMyTurn = isMine,
                        isWaitingForStoryGeneration = false
                    )
                }
            }
            is DomainGameEvent.PlayerJoined -> {
                reduce { copy(players = (players + event.userId).distinct()) }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Player joined: ${event.userId}"))
                }
            }
            is DomainGameEvent.PlayerLeft -> {
                reduce { copy(players = players.filterNot { it == event.userId }) }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Player left: ${event.userId}"))
                }
            }
            is DomainGameEvent.PlayerReady -> {
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Player ${event.userId} ready=${event.ready}"))
                }
            }
            is DomainGameEvent.StoryChunk -> {
                reduce {
                    val merged = (storyChunks + (event.seq to event.chunk))
                        .distinctBy { it.first }
                        .sortedBy { it.first }
                    val combined = merged.joinToString(separator = "") { it.second }
                    copy(storyChunks = merged, storySoFar = combined)
                }
            }
            is DomainGameEvent.StoryComplete -> {
                reduce {
                    copy(storyFullText = event.fullText, storySoFar = event.fullText, isWaitingForStoryGeneration = false)
                }
            }
            DomainGameEvent.WaitingForStoryGeneration -> {
                reduce {
                    copy(isWaitingForStoryGeneration = true, storyChunks = emptyList(), storySoFar = "", storyFullText = null)
                }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Waiting for story generation..."))
                }
            }
            DomainGameEvent.GameStarted -> {
                reduce { copy(isGameStarted = true) }
            }
            DomainGameEvent.SessionDeleted -> {
                // This might need a navigation effect to go back to home screen
            }
        }
    }
}
