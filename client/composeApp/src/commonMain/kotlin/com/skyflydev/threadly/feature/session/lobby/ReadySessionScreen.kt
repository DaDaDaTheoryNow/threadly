package com.skyflydev.threadly.feature.session.lobby

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ElevatedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.skyflydev.threadly.feature.session.lobby.view_model.SessionLobbyViewModel
import com.skyflydev.threadly.feature.session.view_model.SessionIntent
import com.skyflydev.threadly.feature.session.view_model.SessionUiEffect
import kotlinx.coroutines.flow.collectLatest


@Composable
fun ReadySessionScreen(
    onNavigateToHomeScreen: () -> Unit,
    onNavigateToGameScreen: () -> Unit,
    viewModel: SessionLobbyViewModel,
) {
    val state = viewModel.state.collectAsStateWithLifecycle().value
    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(Unit) {
        viewModel.effects.collectLatest { event ->
            when (event) {
                SessionUiEffect.SessionLeave -> {
                    onNavigateToHomeScreen()
                }
                is SessionUiEffect.ShowMessage -> {
                    snackbarHostState.showSnackbar(event.text)
                }
                is SessionUiEffect.NavigateToGameScreen -> {
                    onNavigateToGameScreen()
                }

                SessionUiEffect.NavigateToHomeScreen -> {
                    onNavigateToHomeScreen()
                }
            }
        }
    }

    Scaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) }
    ) { innerPadding ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding),
            contentAlignment = Alignment.Center
        ) {
            Column(horizontalAlignment = Alignment.CenterHorizontally) {
                 state.currentSession?.users?.forEach { userId ->
                     Text(text = "User: $userId")
                 }

                Spacer(modifier = Modifier.height(16.dp))

                 ElevatedButton(
                     onClick = { viewModel.sendIntent(SessionIntent.ToggleReady) }
                 ) {
                     Text(if (state.isUserReady) "Cancel Ready" else "Ready")
                 }

                Spacer(modifier = Modifier.height(16.dp))

                ElevatedButton(
                    onClick = {
                        viewModel.sendIntent(SessionIntent.LeaveSession)
                    }
                ) {
                    Text("Leave Session")
                }

                // TODO: 3. (опционально) Показать тему/настройки сессии
                 Text("Theme: ${state.currentSession?.theme}")
                 Text("Max Rounds: ${state.currentSession?.maxRounds}")

                // TODO: 4. (опционально) Если все готовы — показать кнопку "Начать игру"
                // if (state.allPlayersReady) {
                     ElevatedButton(onClick = { viewModel.sendIntent(SessionIntent.StartGameAsHost) }) {
                         Text("Start Game")
                     }
                // }
            }
        }
    }
}
