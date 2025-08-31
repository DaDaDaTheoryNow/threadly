package com.skyflydev.threadly.feature.session.view_model

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.skyflydev.threadly.core.common.error.toMessage
import com.skyflydev.threadly.core.common.result.onError
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.data.game.dto.GameEvent
import com.skyflydev.threadly.data.game.repository.GameRepository
import com.skyflydev.threadly.data.sessions.repository.SessionsRepository
import com.skyflydev.threadly.feature.session.view_model.SessionUiEffect.*
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch


class SessionViewModel(
    private val sessionId: String,
    private val userId: String,
    private val sessionsRepository: SessionsRepository,
    private val gameRepository: GameRepository,
) : ViewModel() {
    private val _state = MutableStateFlow<SessionState>(SessionState())
    val state = _state.asStateFlow()

    private val _effects = MutableSharedFlow<SessionUiEffect>(extraBufferCapacity = 8)
    val effects = _effects.asSharedFlow()

    init {
        sendIntent(SessionIntent.LoadSessionDetails)
        sendIntent(SessionIntent.ObserveGameEvents)
    }

    fun sendIntent(intent: SessionIntent) {
        when (intent) {
            SessionIntent.LeaveSession -> handleLeaveSession()
            SessionIntent.ToggleReady -> handleToggleReady()
            SessionIntent.LoadSessionDetails -> handleLoadSessionData()
            SessionIntent.ObserveGameEvents -> observeGameEvents()

            is SessionIntent.StartGameAsHost -> handleStartGameAsHost()
            is SessionIntent.SubmitMessage -> handleSubmitMessage(intent)
        }
    }

    private fun handleLoadSessionData() {
        viewModelScope.launch {
            sessionsRepository
                .getSessionDetails(sessionId)
                .onSuccess { session ->
                    reduce { copy(currentSession = session) }
                }
                .onError { error ->
                    _effects.emit(SessionUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    private fun handleLeaveSession() {
        viewModelScope.launch {
            sessionsRepository
                .leaveSession(sessionId)
                .onSuccess {
                    _effects.emit(SessionUiEffect.SessionLeave)
                }
                .onError { error ->
                    _effects.emit(SessionUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    private fun handleToggleReady() {
        viewModelScope.launch {
            sessionsRepository
                .setReady(!_state.value.isUserReady, sessionId)
                .onSuccess {
                    reduce { copy(isUserReady = !_state.value.isUserReady) }
                }
                .onError { error ->
                    _effects.emit(SessionUiEffect.ShowMessage(error.toMessage()))
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

    private fun handleStartGameAsHost() {
        viewModelScope.launch {
            sessionsRepository.startGame(sessionId)
                .onSuccess {
                    // Game Started
                }
                .onError { error ->
                    _effects.emit(SessionUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    private fun handleSubmitMessage(intent: SessionIntent.SubmitMessage) {
        viewModelScope.launch {
            gameRepository.submitMessage(sessionId, intent.content)
                .onSuccess {
                    reduce { copy(lastPlayerMessage = null, isMyTurn = false) }
                }
                .onError { error ->
                    _effects.emit(SessionUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    private fun reduce(reducer: SessionState.() -> SessionState) {
        _state.update { it.reducer() }
    }

    private fun reduceGameEvent(event: GameEvent) {
        when (event) {
            is GameEvent.Error -> {
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Server error: ${event.message}"))
                }
            }
            GameEvent.GameFinished -> {
                reduce { copy(isGameFinished = true) }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Game finished"))
                }
            }
            is GameEvent.LastPlayerMessage -> {
                reduce {
                    copy(lastPlayerMessage = event.content)
                }
            }

            is GameEvent.NewTurn -> {
                val isMine = event.userId == userId
                reduce {
                    copy(
                        currentTurnUserId = event.userId,
                        isMyTurn = isMine,
                        isWaitingForStoryGeneration = false
                    )
                }
            }
            is GameEvent.PlayerJoined -> {
                reduce { copy(players = (players + event.userId).distinct()) }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Player joined: ${event.userId}"))
                }
            }
            is GameEvent.PlayerLeft -> {
                reduce { copy(players = players.filterNot { it == event.userId }) }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Player left: ${event.userId}"))
                }
            }
            is GameEvent.PlayerReady -> {
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Player ${event.userId} ready=${event.ready}"))
                }
            }

            // incoming incremental chunk — собираем и обновляем storySoFar
            is GameEvent.StoryChunk -> {
                reduce {
                    // объединяем старые чанки + новый, уникализируем по seq, сортируем по seq
                    val merged = (storyChunks + (event.seq to event.chunk))
                        .distinctBy { it.first }
                        .sortedBy { it.first }
                    // склеиваем в одну строку — используем прямую конкатенацию, можно добавить пробел/символы при необходимости
                    val combined = merged.joinToString(separator = "") { it.second }
                    copy(storyChunks = merged, storySoFar = combined)
                }
            }

            is GameEvent.StoryComplete -> {
                reduce {
                    copy(storyFullText = event.fullText, storySoFar = event.fullText, isWaitingForStoryGeneration = false)
                }
//                viewModelScope.launch {
//                    _effects.emit(SessionUiEffect.ShowFinalStory(event.storyId, event.fullText))
//                }
            }

            GameEvent.WaitingForStoryGeneration -> {
                reduce {
                    copy(isWaitingForStoryGeneration = true, storyChunks = emptyList(), storySoFar = "", storyFullText = null)
                }
                viewModelScope.launch {
                    _effects.emit(ShowMessage("Waiting for story generation..."))
                }
            }

            GameEvent.GameStarted -> {
                reduce { copy(isGameStarted = true) }
                viewModelScope.launch {
                    _effects.emit(NavigateToGameScreen(sessionId))
                }
            }

            GameEvent.SessionDeleted -> {
                viewModelScope.launch {
                    _effects.emit(NavigateToHomeScreen)
                }
            }
        }
    }
}
