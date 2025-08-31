package com.skyflydev.threadly.feature.session.navigation

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import androidx.navigation.toRoute
import com.skyflydev.threadly.feature.session.GameScreen
import com.skyflydev.threadly.feature.session.ReadySessionScreen
import com.skyflydev.threadly.feature.session.create_session.CreateSessionScreen
import com.skyflydev.threadly.feature.session.view_model.SessionViewModel
import kotlinx.serialization.Serializable
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.parameter.parametersOf

@Serializable
data class SessionRoute(
    val sessionId: String,
    val userId: String
)

@Serializable
data object CreateSessionRoute

fun NavGraphBuilder.createSessionScreen(
    onNavigateToSessionScreen: (sessionId: String, hostUserId: String) -> Unit,
    ) {
    composable<CreateSessionRoute> {
        CreateSessionScreen(
            onNavigateToSessionScreen = onNavigateToSessionScreen
        )
    }
}

fun NavGraphBuilder.sessionScreen(
    onNavigateToHomeScreen: () -> Unit,
) {
    composable<SessionRoute> {
        val sessionId = it.toRoute<SessionRoute>().sessionId
        val userId = it.toRoute<SessionRoute>().userId

        val viewModel = koinViewModel<SessionViewModel>(
            parameters = { parametersOf(sessionId, userId) }
        )

        var isReadyScreen: Boolean by remember { mutableStateOf(true) }

        if (isReadyScreen)
        ReadySessionScreen(
            onNavigateToHomeScreen = onNavigateToHomeScreen,
            onNavigateToGameScreen = {
                isReadyScreen = false
            },
            viewModel = viewModel,
        )

        if (!isReadyScreen) GameScreen(
            viewModel = viewModel,
        )
    }
}