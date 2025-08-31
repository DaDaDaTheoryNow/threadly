package com.skyflydev.threadly.feature.session.navigation

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import androidx.navigation.toRoute
import com.skyflydev.threadly.feature.game.navigation.GameRoute
import com.skyflydev.threadly.feature.session.create.CreateSessionScreen
import com.skyflydev.threadly.feature.session.lobby.ReadySessionScreen
import com.skyflydev.threadly.feature.session.lobby.view_model.SessionLobbyViewModel
import kotlinx.serialization.Serializable
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.parameter.parametersOf

@Serializable
data class SessionLobbyRoute(
    val sessionId: String,
    val userId: String,
    val isHost: Boolean
)

@Serializable
data object CreateSessionRoute

fun NavGraphBuilder.createSessionScreen(
    navController: NavController
) {
    composable<CreateSessionRoute> {
        CreateSessionScreen(
            onNavigateToSessionScreen = { sessionId, hostUserId ->
                navController.navigate(SessionLobbyRoute(
                    sessionId = sessionId,
                    userId = hostUserId,
                    isHost = true
                ))
            }
        )
    }
}

fun NavGraphBuilder.sessionLobbyScreen(
    navController: NavController
) {
    composable<SessionLobbyRoute> {
        val route = it.toRoute<SessionLobbyRoute>()

        val viewModel = koinViewModel<SessionLobbyViewModel>(
            parameters = { parametersOf(route.sessionId, route.userId) }
        )

        ReadySessionScreen(
            onNavigateToHomeScreen = { navController.popBackStack() },
            onNavigateToGameScreen = {
                 navController.navigate(GameRoute(
                    sessionId = route.sessionId,
                    userId = route.userId,
                    isHost = route.isHost
                ))
            },
            viewModel = viewModel,
        )
    }
}