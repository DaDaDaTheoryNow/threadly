package com.skyflydev.threadly.data.sessions.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.common.result.map
import com.skyflydev.threadly.core.model.Session
import com.skyflydev.threadly.data.sessions.datasources.network.SessionsNetworkDataSource
import com.skyflydev.threadly.data.sessions.dto.response.SessionPlayerDto
import com.skyflydev.threadly.data.sessions.dto.response.SessionCreatedDto
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

    override suspend fun startGame(sessionId: String): AppResult<Unit, AppError> {
        return network.startGame(sessionId)
    }

    override suspend fun createSession(
        theme: String,
        maxRounds: Int
    ): AppResult<SessionCreatedDto, AppError> {
        return network.createSession(theme, maxRounds)
    }

    override suspend fun joinSession(sessionId: String): AppResult<SessionPlayerDto, AppError> {
        return network.joinSession(sessionId)
    }

    override suspend fun leaveSession(
        sessionId: String
    ): AppResult<Unit, AppError> {
        return network.leaveSession(sessionId)
    }

    override suspend fun setReady(
        value: Boolean,
        sessionId: String
    ): AppResult<SessionPlayerDto, AppError> {
        return network.setReady(value, sessionId)
    }
}