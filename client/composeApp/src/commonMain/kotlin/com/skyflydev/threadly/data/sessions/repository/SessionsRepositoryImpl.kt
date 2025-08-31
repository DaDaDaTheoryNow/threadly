package com.skyflydev.threadly.data.sessions.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.common.result.map
import com.skyflydev.threadly.core.model.Player
import com.skyflydev.threadly.core.model.Session
import com.skyflydev.threadly.data.sessions.datasources.network.SessionsNetworkDataSource
import com.skyflydev.threadly.data.sessions.dto.response.SessionEvent
import com.skyflydev.threadly.data.sessions.mappers.toEntity
import kotlinx.coroutines.flow.Flow

class SessionsRepositoryImpl(
    private val network: SessionsNetworkDataSource,
) : SessionsRepository {
    override fun sessionsEventsFlow(): Flow<SessionEvent> = network.observeSessionsEvents()

    override suspend fun getSessionsSnapshot(): AppResult<List<Session>, AppError> {
        return network.fetchSessions().map { sessionsDtoList ->
            sessionsDtoList.map {
                it.toEntity()
            }
        }
    }

    override suspend fun getSessionDetails(sessionId: String): AppResult<Session, AppError> {
        return network.fetchSessionDetails(sessionId).map { it.toEntity() }
    }

    override suspend fun createSession(
        theme: String,
        maxRounds: Int
    ): AppResult<String, AppError> {
        return network.createSession(theme, maxRounds).map { it.sessionId }
    }

    override suspend fun joinSession(sessionId: String): AppResult<Player, AppError> {
        return network.joinSession(sessionId).map { it.toEntity() }
    }

    override suspend fun leaveSession(
        sessionId: String
    ): AppResult<Unit, AppError> {
        return network.leaveSession(sessionId)
    }

    override suspend fun setReady(
        value: Boolean,
        sessionId: String
    ): AppResult<Player, AppError> {
        return network.setReady(value, sessionId).map { it.toEntity() }
    }
}