package com.skyflydev.threadly.feature.auth.view_model

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.skyflydev.threadly.core.common.error.toMessage
import com.skyflydev.threadly.core.common.result.onError
import com.skyflydev.threadly.data.auth.repository.AuthRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class AuthViewModel(
    private val authRepository: AuthRepository,
) : ViewModel() {

    private val _state = MutableStateFlow(AuthState())
    val state = _state.asStateFlow()

    val isUserLoggedIn = authRepository.isUserLoggedIn

    fun sendIntent(intent: AuthIntent) {
        when (intent) {
            is AuthIntent.UsernameInput -> reduce { copy(username = intent.value) }
            is AuthIntent.PasswordInput -> reduce { copy(password = intent.value) }
            is AuthIntent.SignIn -> handleSignIn(intent.username, intent.password)
            is AuthIntent.SignUp -> handleSignUp(intent.username, intent.password)
        }
    }

    private fun handleSignIn(username: String, password: String) {
        viewModelScope.launch {
            reduce { copy(isLoading = true, errorMessage = null) }

            authRepository.signIn(username, password)
                .onError { error ->
                    reduce { copy(errorMessage = error.toMessage()) }
                }

            reduce { copy(isLoading = false) }
        }
    }

    private fun handleSignUp(username: String, password: String) {
        viewModelScope.launch {
            reduce { copy(isLoading = true, errorMessage = null) }

            authRepository.signUp(username, password)
                .onError { error ->
                    reduce { copy(errorMessage = error.toMessage()) }
                }

            reduce { copy(isLoading = false) }
        }
    }

    // ---------------------------
    // Reducer
    // ---------------------------
    private fun reduce(reducer: AuthState.() -> AuthState) {
        _state.update { it.reducer() }
    }
}
