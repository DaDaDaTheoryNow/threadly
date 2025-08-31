package com.skyflydev.threadly.feature.session.create

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.flow.collectLatest
import org.koin.compose.viewmodel.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CreateSessionScreen(
    onNavigateToSessionScreen: (sessionId: String, hostUserId: String) -> Unit,
    onBack: () -> Unit = {},
    viewModel: CreateSessionViewModel = koinViewModel()
) {
    LaunchedEffect(Unit) {
        viewModel.effects.collectLatest { event ->
            when (event) {
                is NewSessionUiEffect.NavigateToSessionScreen -> onNavigateToSessionScreen(event.sessionId, event.userId)
            }
        }
    }

    var theme by remember { mutableStateOf("") }

    Scaffold(
        topBar = {
            TopAppBar(title = { Text("Create a session") })
        }
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp)
                .padding(innerPadding),
            verticalArrangement = Arrangement.Top
        ) {
            OutlinedTextField(
                value = theme,
                onValueChange = {
                    theme = it
//                    if (showError && it.isNotBlank()) showError = false
                },
                modifier = Modifier.fillMaxWidth(),
                label = { Text("Тема") },
                singleLine = true,
//                isError = showError,
                keyboardOptions = KeyboardOptions.Default.copy(imeAction = ImeAction.Done),
//                keyboardActions = KeyboardActions(onDone = {
//                    if (theme.isNotEmpty()) {
//                        viewModel.sendIntent(NewSessionIntent.CreateSession(theme.trim()))
//                    }
//
////                    focusManager.clearFocus()
////                    if (theme.trim().isEmpty()) {
////                        showError = true
////                    } else if (!isLoading) {
////                        onCreate(theme.trim())
////                    }
//                })
            )

//            if (showError) {
//                Text(
//                    text = "Тема не может быть пустой",
//                    style = MaterialTheme.typography.bodySmall,
//                    color = MaterialTheme.colorScheme.error,
//                    modifier = Modifier.padding(top = 6.dp, start = 4.dp)
//                )
//            }

            Spacer(modifier = Modifier.height(20.dp))

            Button(
                onClick = {
//                    if (theme.trim().isEmpty()) {
//                        showError = true
//                    } else {
//                        focusManager.clearFocus()
//                        onCreate(theme.trim())
//                    }

                    if (theme.isNotEmpty()) {
                        viewModel.sendIntent(NewSessionIntent.CreateSession(theme.trim()))
                    }
                },
                enabled = theme.trim().isNotEmpty(),
                modifier = Modifier.fillMaxWidth()
            ) {
                Text("Создать")
            }

            Spacer(modifier = Modifier.height(8.dp))
//
//            TextButton(onClick = onBack, modifier = Modifier.fillMaxWidth()) {
//                Text("Отмена")
//            }
        }
    }
}