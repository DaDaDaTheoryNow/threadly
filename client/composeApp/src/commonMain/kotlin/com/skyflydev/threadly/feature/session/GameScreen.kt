package com.skyflydev.threadly.feature.session

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.skyflydev.threadly.feature.session.view_model.SessionIntent
import com.skyflydev.threadly.feature.session.view_model.SessionViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle

@Composable
fun GameScreen(
    viewModel: SessionViewModel,
    onBack: () -> Unit = {}
) {
    val state = viewModel.state.collectAsStateWithLifecycle().value
    var input by remember { mutableStateOf("") }
    val scrollState = rememberScrollState()

    Column(modifier = Modifier
        .fillMaxSize()
        .padding(16.dp)) {

        // Header: players / current turn
        Text(text = "Players: ${state.players.joinToString()}", style = MaterialTheme.typography.bodyMedium)
        Spacer(modifier = Modifier.height(8.dp))
        Text(text = "Current turn: ${state.currentTurnUserId ?: "—"}", style = MaterialTheme.typography.bodyMedium)
        Spacer(modifier = Modifier.height(12.dp))

        // Last player message — показываем, если есть (видим только то, что пришло)
        state.lastPlayerMessage?.let { last ->
            Text(text = "Предложение прошлого игрока:", style = MaterialTheme.typography.labelMedium)
            Text(text = last, style = MaterialTheme.typography.bodyLarge)
            Spacer(modifier = Modifier.height(12.dp))
        }

        // Story in progress — вертикальный скролл, обновляется по мере прихода чанков
        Text(text = "История (по мере генерации):", style = MaterialTheme.typography.labelMedium)
        Spacer(modifier = Modifier.height(8.dp))
        Box(modifier = Modifier
            .weight(1f)
            .fillMaxWidth()
            .verticalScroll(scrollState)) {
            Text(text = if (state.storySoFar.isNotEmpty()) state.storySoFar else "Пока ничего...", style = MaterialTheme.typography.bodyLarge)
        }

        Spacer(modifier = Modifier.height(12.dp))

        // Input: появляется только если это наш ход
        if (state.isMyTurn) {
            OutlinedTextField(
                value = input,
                onValueChange = { input = it },
                modifier = Modifier.fillMaxWidth(),
                label = { Text("Ваше предложение") },
                singleLine = false,
                maxLines = 4
            )
            Spacer(modifier = Modifier.height(8.dp))
            Row {
                ElevatedButton(onClick = {
                    val trimmed = input.trim()
                    if (trimmed.isNotEmpty()) {
                        viewModel.sendIntent(SessionIntent.SubmitMessage(trimmed))
                        input = "" // очистить поле при успехе — сервер пришлёт NewTurn
                    }
                }) {
                    Text("Отправить")
                }
                Spacer(modifier = Modifier.width(8.dp))
                TextButton(onClick = { input = "" }) {
                    Text("Отменить")
                }
            }
        } else {
            // если не наш ход — показать подсказку
            if (state.currentTurnUserId != null) {
                Text("Ждем хода: ${state.currentTurnUserId}")
            } else {
                Text("Ожидание следующего хода...")
            }
        }

        Spacer(modifier = Modifier.height(8.dp))

        if (state.isWaitingForStoryGeneration) {
            Text("Ожидаем генерацию истории...")
        }

        // Кнопка назад
        Spacer(modifier = Modifier.height(8.dp))
        TextButton(onClick = onBack) {
            Text("В сессию")
        }
    }
}
