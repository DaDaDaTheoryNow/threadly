package com.skyflydev.threadly.data.sessions.datasources.network

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.common.result.map
import com.skyflydev.threadly.core.common.result.onError
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.core.network.JsonConfig
import com.skyflydev.threadly.core.network.UnauthorizedHandler
import com.skyflydev.threadly.core.network.safeCall
import com.skyflydev.threadly.core.network.safeWebSocket
import com.skyflydev.threadly.data.sessions.dto.request.CreateSessionRequestDto
import com.skyflydev.threadly.data.sessions.dto.request.SetReadyRequestDto
import com.skyflydev.threadly.data.sessions.dto.response.SessionCreatedDto
import com.skyflydev.threadly.data.sessions.dto.response.SessionDto
import com.skyflydev.threadly.data.sessions.dto.response.SessionEvent
import com.skyflydev.threadly.data.sessions.dto.response.SessionPlayerDto
import com.skyflydev.threadly.data.token.AuthDataStore
import io.ktor.client.HttpClient
import io.ktor.client.request.bearerAuth
import io.ktor.client.request.delete
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

interface SessionsNetworkDataSource {
    suspend fun fetchSessions(): AppResult<List<SessionDto>, AppError>
    fun observeSessionsEvents(): Flow<SessionEvent>
    suspend fun fetchSessionDetails(sessionId: String): AppResult<SessionDto, AppError>

    suspend fun createSession(theme: String, maxRounds: Int): AppResult<SessionCreatedDto, AppError>
    suspend fun joinSession(sessionId: String): AppResult<SessionPlayerDto, AppError>
    suspend fun leaveSession(sessionId: String): AppResult<Unit, AppError>
    suspend fun setReady(value: Boolean, sessionId: String): AppResult<SessionPlayerDto, AppError>
}

class SessionsNetworkDataSourceImpl(
    private val authDataStore: AuthDataStore,
    private val unauthorizedHandler: UnauthorizedHandler,
    private val httpClient: HttpClient,
) : SessionsNetworkDataSource {

    private val json = JsonConfig.json

    override suspend fun fetchSessions(): AppResult<List<SessionDto>, AppError> {
        return authDataStore.getAuthData().map { authData ->
            return safeCall<List<SessionDto>> {
                httpClient.get(
                    urlString = "/api/sessions"
                ) {
                    bearerAuth(authData.token)
                }
            }
        }
    }

    override fun observeSessionsEvents(): Flow<SessionEvent> = callbackFlow {
        var wsJob: Job? = null

        authDataStore.getAuthData().map { authData ->
            val wsUrl = "/observe/sessions?token=${authData.token}"

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
                                val event = json.decodeFromString<SessionEvent>(text)
                                trySend(event).isSuccess
                            }
                        }
                    } catch (e: Exception) {
                        // Re-throw to be handled by safeWebSocket catch (so it returns AppResult.Error)
                        throw e
                    }
                }.onSuccess {
                    close()
                }.onError { error ->
                    print("observeSessionsEvents closed with error: $error")
                    close()
                }
            }
        }

        awaitClose {
            wsJob?.cancel()
        }
    }

    override suspend fun fetchSessionDetails(sessionId: String): AppResult<SessionDto, AppError> {
        return authDataStore.getAuthData().map { authData ->
            return safeCall<SessionDto> {
                httpClient.get(
                    urlString = "/api/sessions/$sessionId"
                ) {
                    bearerAuth(authData.token)
                }
            }
        }
    }

    override suspend fun createSession(
        theme: String,
        maxRounds: Int
    ): AppResult<SessionCreatedDto, AppError> {
        return authDataStore
            .getAuthData()
            .map { authData ->
                return safeCall<SessionCreatedDto> {
                    httpClient.post(urlString = "/api/sessions") {
                        bearerAuth(authData.token)
                        setBody(
                            CreateSessionRequestDto(
                                theme,
                                maxRounds
                            )
                        )
                    }
                }
            }
    }

    override suspend fun joinSession(sessionId: String): AppResult<SessionPlayerDto, AppError> {
        return authDataStore
            .getAuthData()
            .map { authData ->
                return safeCall<SessionPlayerDto> {
                    httpClient.post(urlString = "/api/sessions/join") {
                        bearerAuth(authData.token)
                        setBody(mapOf(
                            "session_id" to sessionId,
                        ))
                    }
                }
            }
    }

    override suspend fun leaveSession(sessionId: String): AppResult<Unit, AppError> {
        return authDataStore
            .getAuthData()
            .map { authData ->
                return safeCall<Unit> {
                    httpClient.delete(urlString = "/api/sessions/leave") {
                        bearerAuth(authData.token)
                        setBody(mapOf(
                            "session_id" to sessionId,
                        ))
                    }
                }
            }
    }

    override suspend fun setReady(value: Boolean, sessionId: String): AppResult<SessionPlayerDto, AppError> {
        return authDataStore
            .getAuthData()
            .map { authData ->
                return safeCall<SessionPlayerDto> {
                    httpClient.post(urlString = "/api/sessions/ready") {
                        bearerAuth(authData.token)
                        setBody(
                            SetReadyRequestDto(
                                sessionId = sessionId,
                                isReady = value
                            )
                        )
                    }
                }
            }
    }
}
