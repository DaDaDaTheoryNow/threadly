package com.skyflydev.threadly

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.*
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.skyflydev.threadly.core.designsystem.theme.ThreadlyTheme
import com.skyflydev.threadly.feature.auth.view_model.AuthViewModel
import com.skyflydev.threadly.navigation.RootScreen
import kotlinx.coroutines.delay
import org.jetbrains.compose.ui.tooling.preview.Preview
import org.koin.compose.KoinContext
import org.koin.compose.viewmodel.koinViewModel

@Composable
@Preview
fun App(
    viewModel: AuthViewModel = koinViewModel()
) {
    val isUserLoggedIn by viewModel.isUserLoggedIn.collectAsStateWithLifecycle(null)

    var showSplash by remember { mutableStateOf(true) }

    LaunchedEffect(Unit) {
        delay(1500)
        showSplash = false
    }

    KoinContext {
        ThreadlyTheme {
            Surface(
                color = MaterialTheme.colorScheme.background
            ) {
                when {
                    showSplash -> SplashScreen()
                    isUserLoggedIn != null -> RootScreen(isUserLoggedIn = isUserLoggedIn!!)
                    else -> SplashScreen()
                }
            }
        }
    }
}
