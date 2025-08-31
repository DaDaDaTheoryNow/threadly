package com.skyflydev.threadly.feature.home

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ElevatedButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.skyflydev.threadly.core.model.Session
import com.skyflydev.threadly.feature.home.view_model.HomeIntent
import com.skyflydev.threadly.feature.home.view_model.HomeUiEffect
import com.skyflydev.threadly.feature.home.view_model.HomeViewModel
import kotlinx.coroutines.flow.collectLatest
import org.koin.compose.viewmodel.koinViewModel


@Composable
fun HomeScreen(
    onNavigateToCreateSession: () -> Unit,
    onNavigateToSession: (sessionId: String, userId: String) -> Unit,
    viewModel: HomeViewModel = koinViewModel(),
) {
    val state = viewModel.state.collectAsStateWithLifecycle().value
    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(Unit) {
        viewModel.effects.collectLatest { event ->
            when (event) {
                is HomeUiEffect.NavigateToSession -> onNavigateToSession(
                    event.sessionId,
                    event.userId
                )
                is HomeUiEffect.ShowMessage -> snackbarHostState.showSnackbar(event.text)
            }
        }
    }

    Scaffold(
        topBar = {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
                horizontalArrangement = Arrangement.End
            ) {
                IconButton(
                    onClick = {
                        onNavigateToCreateSession()
                    }
                ) { Text("New") }
                ElevatedButton(onClick = { viewModel.sendIntent(HomeIntent.SignOut) }) {
                    Text("Выйти из аккаунта")
                }
            }
        },
        snackbarHost = { SnackbarHost(snackbarHostState) }
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding)
        ) {
            if (state.sessionsWithUser.isNotEmpty()) {
                Text(
                    text = "Продолжить игру",
                    style = MaterialTheme.typography.titleMedium,
                    modifier = Modifier.padding(16.dp)
                )

                LazyColumn(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp)
                ) {
                    items(state.sessionsWithUser) { session ->
                        SessionCard(
                            session = session,
                            onClick = { viewModel.sendIntent(HomeIntent.JoinSession(it)) },
                            showLeaveButton = true,
                            onLeaveClick = {
                                viewModel.sendIntent(HomeIntent.LeaveSession(it))
                            }
                        )
                    }
                }
            }

            Spacer(modifier = Modifier.height(24.dp))

            if (state.sessionsWithoutUser.isNotEmpty()) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = "Доступные сессии",
                        style = MaterialTheme.typography.titleMedium
                    )
                    ElevatedButton(onClick = {
                        viewModel.sendIntent(HomeIntent.SignOut)
                    }) {
                        Text("Выйти")
                    }
                }

                LazyColumn(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp)
                ) {
                    items(state.sessionsWithoutUser) { session ->
                        SessionCard(
                            session = session,
                            onClick = { viewModel.sendIntent(HomeIntent.JoinSession(it)) }
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun SessionCard(
    session: Session,
    onClick: (sessionId: String) -> Unit,
    showLeaveButton: Boolean = false,
    onLeaveClick: ((sessionId: String) -> Unit)? = null
) {
    Card(
        shape = MaterialTheme.shapes.medium,
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surface),
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = session.theme,
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurface
                )
                Spacer(modifier = Modifier.height(6.dp))
                Text(
                    text = "Игроков: ${session.playersCount}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = "Раундов: ${session.maxRounds}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            Row(verticalAlignment = Alignment.CenterVertically) {
                if (showLeaveButton && onLeaveClick != null) {
                    ElevatedButton(
                        onClick = {
                            onLeaveClick(session.id)
                        },
                        modifier = Modifier.padding(end = 8.dp)
                    ) {
                        Text("Выйти")
                    }
                }

                IconButton(
                    onClick = { onClick(session.id) },
                    modifier = Modifier.background(
                        color = Color.White,
                        shape = CircleShape
                    )
                ) {
                    Icon(
                        imageVector = Icons.AutoMirrored.Filled.ArrowForward,
                        contentDescription = "Перейти",
                        tint = Color.Black
                    )
                }
            }
        }
    }
}
