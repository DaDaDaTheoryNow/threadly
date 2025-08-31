package com.skyflydev.threadly.feature.auth.navigation

import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import com.skyflydev.threadly.feature.auth.AuthScreen
import kotlinx.serialization.Serializable

@Serializable
data object AuthRoute

fun NavGraphBuilder.authScreen() {
    composable<AuthRoute> {
        AuthScreen()
    }
}