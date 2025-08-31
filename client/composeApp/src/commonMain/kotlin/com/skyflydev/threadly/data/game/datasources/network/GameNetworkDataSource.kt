package com.skyflydev.threadly.data.game.datasources.network

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.common.result.map
import com.skyflydev.threadly.core.common.result.onError
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.core.network.JsonConfig
import com.skyflydev.threadly.core.network.UnauthorizedHandler
import com.skyflydev.threadly.core.network.safeCall
import com.skyflydev.threadly.core.network.safeWebSocket
import com.skyflydev.threadly.data.game.dto.GameEvent
import com.skyflydev.threadly.data.token.AuthDataStore
import io.ktor.client.HttpClient
import io.ktor.client.request.bearerAuth
import io.ktor.client.request.get
import io.ktor.client.request.post
import io.ktor.client.request.setBody
import io.ktor.websocket.Frame
import io.ktor.websocket.readText
import kotlinx.coroutines.Job
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch

interface GameNetworkDataSource {
    fun observeGameEvents(sessionId: String): Flow<GameEvent>
    suspend fun submitMessage(sessionId: String, content: String): AppResult<Unit, AppError>
}

class GameNetworkDataSourceImpl(
    private val authDataStore: AuthDataStore,
    private val unauthorizedHandler: UnauthorizedHandler,
    private val httpClient: HttpClient,
) : GameNetworkDataSource {

    private val json = JsonConfig.json

    override fun observeGameEvents(sessionId: String): Flow<GameEvent> = callbackFlow {
        var wsJob: Job? = null

        authDataStore.getAuthData().map { authData ->
            val wsUrl = "/observe/game/${sessionId}?token=${authData.token}"

            wsJob = launch {
                httpClient.safeWebSocket(
                    urlString = wsUrl,
                    unauthorizedHandler = unauthorizedHandler
                ) {
                    try {
                        while (isActive) {
                            val frame = incoming.receive()
                            if (frame is Frame.Text) {
                                val text = frame.readText()
                                val event = json.decodeFromString<GameEvent>(text)
                                trySend(event).isSuccess
                            }
                        }
                    } catch (e: Exception) {
                        // Re-throw to be handled by safeWebSocket catch (so it returns AppResult.Error)
                        throw e
                    }
                }.onSuccess {
                    print("observeGameEvents closed successfully")
                    close()
                }.onError { error ->
                    print("observeGameEvents closed with error: $error")
                    close()
                }
            }
        }

        awaitClose {
            wsJob?.cancel()
        }
    }

    override suspend fun submitMessage(
        sessionId: String,
        content: String,
    ): AppResult<Unit, AppError> {
        return authDataStore.getAuthData().map { authData ->
            return safeCall<Unit> {
                httpClient.post(
                    urlString = "/api/sessions/message"
                ) {
                    bearerAuth(authData.token)
                    setBody(mapOf(
                        "session_id" to sessionId,
                        "content" to content
                    ))
                }
            }
        }
    }
}
