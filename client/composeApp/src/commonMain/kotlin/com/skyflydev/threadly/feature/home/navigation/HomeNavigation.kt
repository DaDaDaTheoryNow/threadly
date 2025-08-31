package com.skyflydev.threadly.feature.home.navigation

import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import com.skyflydev.threadly.feature.home.HomeScreen
import kotlinx.serialization.Serializable

@Serializable
data object HomeRoute

fun NavGraphBuilder.homeScreen(
    onNavigateToSession: (sessionId: String, userId: String) -> Unit,
    onNavigateToCreateSession: () -> Unit,
) {
    composable<HomeRoute> {
        HomeScreen(
            onNavigateToCreateSession = onNavigateToCreateSession,
            onNavigateToSession = onNavigateToSession
        )
    }
}