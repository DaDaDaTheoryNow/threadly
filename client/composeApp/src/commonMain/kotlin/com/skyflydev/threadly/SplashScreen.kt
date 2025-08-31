package com.skyflydev.threadly

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.scaleIn
import androidx.compose.animation.scaleOut
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.layout.*
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.delay
import kotlin.random.Random

@Composable
fun SplashScreen() {
    var titleState by remember { mutableStateOf("Loading...") }
    val slogans = listOf(
        "Talk. Play. Connect.",
        "Let the story unfold...",
        "Threads that connect us all",
        "Every session is a new journey",
        "Shaping moments, together",
    )

    var currentIndex by remember { mutableStateOf(Random.nextInt(1, 4)) }

    LaunchedEffect(Unit) {
        delay(500)
        currentIndex = 0
        titleState = "Threadly"
    }

    Surface(
        modifier = Modifier.fillMaxSize(),
        color = MaterialTheme.colorScheme.background
    ) {
        Box(
            modifier = Modifier.fillMaxSize(),
            contentAlignment = Alignment.Center
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center
            ) {
                AnimatedContent(
                    targetState = titleState,
                    transitionSpec = {
                        (fadeIn(animationSpec = tween(800)) + scaleIn(initialScale = 0.8f)) togetherWith
                                (fadeOut(animationSpec = tween(800)) + scaleOut(targetScale = 1.2f))
                    },
                    label = "AppTitleAnimation"
                ) { title ->
                    Text(
                        text = title,
                        style = MaterialTheme.typography.headlineMedium.copy(
                            fontWeight = FontWeight.Bold,
                            fontSize = 28.sp
                        ),
                        color = MaterialTheme.colorScheme.onBackground
                    )
                }

                Spacer(modifier = Modifier.height(8.dp))

                AnimatedContent(
                    targetState = slogans[currentIndex],
                    transitionSpec = {
                        (fadeIn(animationSpec = tween(800))) togetherWith
                                (fadeOut(animationSpec = tween(800)))
                    },
                    label = "SloganAnimation"
                ) { slogan ->
                    Text(
                        text = slogan,
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.7f)
                    )
                }
            }
        }
    }
}
