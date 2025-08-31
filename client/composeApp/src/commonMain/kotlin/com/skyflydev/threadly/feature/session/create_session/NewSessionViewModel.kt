package com.skyflydev.threadly.feature.session.create_session

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.data.sessions.repository.SessionsRepository
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.launch


class CreateSessionViewModel(
    private val sessionsRepository: SessionsRepository
) : ViewModel() {
    private val _effects = MutableSharedFlow<NewSessionUiEffect>(extraBufferCapacity = 8)
    val effects = _effects.asSharedFlow()

    fun sendIntent(intent: NewSessionIntent) {
        when (intent) {
            is NewSessionIntent.CreateSession -> handleCreateSession(intent)
        }
    }

    private fun handleCreateSession(intent: NewSessionIntent.CreateSession) {
       viewModelScope.launch {
           sessionsRepository
               .createSession(intent.theme, 2)
               .onSuccess {
                   _effects.emit(
                       NewSessionUiEffect.NavigateToSessionScreen(it.sessionId, it.hostUserId)
                   )
               }
       }
    }
}
