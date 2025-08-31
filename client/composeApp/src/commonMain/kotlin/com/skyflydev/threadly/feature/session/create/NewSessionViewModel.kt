package com.skyflydev.threadly.feature.session.create

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.data.sessions.repository.SessionsRepository
import com.skyflydev.threadly.data.token.AuthDataStore
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.launch


class NewSessionViewModel(
    private val sessionsRepository: SessionsRepository,
    private val authDataStore: AuthDataStore
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
           val authDataResult = authDataStore.getAuthData()
           val userId = authDataResult.map { it.userId }.getOrNull()

           if (userId == null) {
               // Handle error, maybe emit an effect to show a message
               return@launch
           }

           sessionsRepository
               .createSession(intent.theme, 2)
               .onSuccess { sessionId ->
                   _effects.emit(
                       NewSessionUiEffect.NavigateToSessionScreen(sessionId, userId)
                   )
               }
       }
    }
}
