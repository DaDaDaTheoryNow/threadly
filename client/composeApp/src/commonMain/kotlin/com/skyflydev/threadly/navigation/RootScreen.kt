package com.skyflydev.threadly.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.rememberNavController
import com.skyflydev.threadly.feature.auth.navigation.AuthRoute
import com.skyflydev.threadly.feature.auth.navigation.authScreen
import com.skyflydev.threadly.feature.game.navigation.gameScreen
import com.skyflydev.threadly.feature.home.navigation.HomeRoute
import com.skyflydev.threadly.feature.home.navigation.homeScreen
import com.skyflydev.threadly.feature.session.navigation.CreateSessionRoute
import com.skyflydev.threadly.feature.session.navigation.SessionLobbyRoute
import com.skyflydev.threadly.feature.session.navigation.createSessionScreen
import com.skyflydev.threadly.feature.session.navigation.sessionLobbyScreen

@Composable
fun RootScreen(
    isUserLoggedIn: Boolean
) {
    val navController = rememberNavController()
    NavHost(
        navController = navController,
        startDestination = if (isUserLoggedIn) {
            HomeRoute
        } else {
            AuthRoute
        }
    ) {
        authScreen()

        homeScreen(
            onNavigateToCreateSession = {
                navController.navigate(CreateSessionRoute)
            },
            onNavigateToSession = { sessionId, userId ->
                navController.navigate(SessionLobbyRoute(
                    sessionId = sessionId,
                    userId = userId,
                    isHost = false
                ))
            }
        )

        sessionLobbyScreen(
            navController = navController
        )

        gameScreen(
            onBack = { navController.popBackStack() }
        )

        createSessionScreen(
            navController = navController
        )
    }
}