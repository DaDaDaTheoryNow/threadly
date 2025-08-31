package com.skyflydev.threadly.feature.game.navigation

import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import androidx.navigation.toRoute
import com.skyflydev.threadly.feature.game.GameScreen
import com.skyflydev.threadly.feature.game.view_model.GameViewModel
import kotlinx.serialization.Serializable
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.parameter.parametersOf

@Serializable
data class GameRoute(
    val sessionId: String,
    val userId: String,
    val isHost: Boolean
)

fun NavGraphBuilder.gameScreen(
    onBack: () -> Unit
) {
    composable<GameRoute> {backStackEntry ->
        val route = backStackEntry.toRoute<GameRoute>()
        val viewModel: GameViewModel = koinViewModel(
            parameters = { parametersOf(route.sessionId, route.userId, route.isHost) }
        )
        GameScreen(
            viewModel = viewModel,
            onBack = onBack
        )
    }
}
