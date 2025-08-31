package com.skyflydev.threadly.feature.home.view_model

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.skyflydev.threadly.core.common.error.toMessage
import com.skyflydev.threadly.core.common.result.onError
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.data.auth.repository.AuthRepository
import com.skyflydev.threadly.data.sessions.dto.response.SessionEvent
import com.skyflydev.threadly.data.sessions.mappers.toEntity
import com.skyflydev.threadly.data.sessions.repository.SessionsRepository
import com.skyflydev.threadly.data.token.AuthDataStore
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class HomeViewModel(
    private val sessionsRepository: SessionsRepository,
    private val authRepository: AuthRepository,
    private val authDataStore: AuthDataStore,
) : ViewModel() {

    private val _state = MutableStateFlow(HomeState())
    val state = _state.asStateFlow()

    private val _effects = MutableSharedFlow<HomeUiEffect>(extraBufferCapacity = 8)
    val effects = _effects.asSharedFlow()

    private var currentUserId: String? = null

    init {
        bootstrap()
    }

    private fun bootstrap() {
        viewModelScope.launch {
            authDataStore.getAuthData()
                .onSuccess { authData ->
                    currentUserId = authData.userId
                    sendIntent(HomeIntent.LoadSessions)
                    sendIntent(HomeIntent.ObserveSessionEvents)
                }
                .onError { error ->
                    _effects.emit(HomeUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }


    fun sendIntent(intent: HomeIntent) {
        when (intent) {
            is HomeIntent.JoinSession -> handleJoinSession(intent)
            HomeIntent.LoadSessions -> handleLoadSessions()
            HomeIntent.ObserveSessionEvents -> observeSessionEvents()
            is HomeIntent.LeaveSession -> handleLeaveSession(intent)
            HomeIntent.SignOut -> handleSignOut()
        }
    }

    private fun handleJoinSession(intent: HomeIntent.JoinSession) {
        viewModelScope.launch {
            _effects.emit(HomeUiEffect.ShowMessage("Connecting to session..."))

            sessionsRepository.joinSession(intent.sessionId)
                .onSuccess { result ->
                    _effects.emit(HomeUiEffect.NavigateToSession(intent.sessionId, result.userId))
                }
                .onError { error ->
                    _effects.emit(HomeUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    private fun handleLoadSessions() {
        viewModelScope.launch {
            sessionsRepository.getSessionsSnapshot()
                .onSuccess { sessions ->
                    reduce {
                        copy(sessions = sessions)
                    }

                    updateWithAndWithoutSession()
                }
                .onError { error ->
                    _effects.emit(HomeUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    private fun observeSessionEvents() {
        sessionsRepository.sessionsEventsFlow()
            .onEach { event ->
                print("SessionEven: $event")
                reduceSessionEvent(event)
                updateWithAndWithoutSession()
            }
            .launchIn(viewModelScope)
    }

    private fun handleLeaveSession(intent: HomeIntent.LeaveSession) {
        viewModelScope.launch {
            sessionsRepository
                .leaveSession(intent.sessionId)
                .onError { error ->
                    _effects.emit(HomeUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    private fun handleSignOut() {
        viewModelScope.launch {
            authRepository.signOut()
                .onError { error ->
                    _effects.emit(HomeUiEffect.ShowMessage(error.toMessage()))
                }
        }
    }

    fun updateWithAndWithoutSession() {
        val userId = currentUserId ?: return

        reduce { copy(
            sessionsWithUser = sessions.filter { it.users.any { it.userId == userId } },
            sessionsWithoutUser = sessions.filterNot { it.users.any { it.userId == userId } }
        ) }
    }

    private fun reduce(reducer: HomeState.() -> HomeState) {
        _state.update { it.reducer() }
    }

    private fun reduceSessionEvent(event: SessionEvent) {
        when (event) {
            is SessionEvent.Created -> {
                reduce {
                    copy(
                        sessions = sessions + event.toEntity()
                    )
                }
            }

            is SessionEvent.UpdatePlayers -> {
                reduce {
                    copy(
                        sessions = sessions.map { session ->
                            if (session.id == event.sessionId) {
                                session.copy(
                                    users = event.users.map { it.toEntity() },
                                    playersCount = event.users.size
                                )
                            } else {
                                session
                            }
                        }
                    )
                }
            }

            is SessionEvent.Started -> {
                reduce {
                    copy(
                        sessions = sessions.filter { it.id != event.sessionId }
                    )
                }
            }

            is SessionEvent.Deleted -> {
                reduce {
                    copy(
                        sessions = sessions.filter { it.id != event.sessionId }
                    )
                }
            }
        }
    }
}