package com.skyflydev.threadly.feature.session.lobby.view_model

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.skyflydev.threadly.core.common.error.toMessage
import com.skyflydev.threadly.core.common.result.onError
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.data.sessions.repository.SessionsRepository
import com.skyflydev.threadly.feature.session.view_model.SessionIntent
import com.skyflydev.threadly.feature.session.view_model.SessionState
import com.skyflydev.threadly.feature.session.view_model.SessionUiEffect
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class SessionLobbyViewModel(
    private val sessionId: String,
    private val userId: String,
    private val sessionsRepository: SessionsRepository
) : ViewModel() {
    private val _state = MutableStateFlow(SessionState())
    val state = _state.asStateFlow()

    private val _effects = MutableSharedFlow<SessionUiEffect>(extraBufferCapacity = 8)
    val effects = _effects.asSharedFlow()

    init {
        sendIntent(SessionIntent.LoadSessionDetails)
    }

    fun sendIntent(intent: SessionIntent) {
        when (intent) {
            SessionIntent.LeaveSession -> handleLeaveSession()
            SessionIntent.ToggleReady -> handleToggleReady()
            SessionIntent.LoadSessionDetails -> handleLoadSessionData()
            is SessionIntent.StartGameAsHost -> handleStartGameAsHost()
            else -> {
                // No-op for game related intents
            }
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

    private fun handleStartGameAsHost() {
        viewModelScope.launch {
            _effects.emit(SessionUiEffect.NavigateToGameScreen(sessionId))
        }
    }

    private fun reduce(reducer: SessionState.() -> SessionState) {
        _state.update { it.reducer() }
    }
}
