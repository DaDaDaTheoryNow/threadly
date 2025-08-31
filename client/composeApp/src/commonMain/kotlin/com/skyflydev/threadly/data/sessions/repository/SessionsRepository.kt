package com.skyflydev.threadly.data.sessions.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.model.Session
import com.skyflydev.threadly.data.sessions.dto.response.SessionCreatedDto
import com.skyflydev.threadly.data.sessions.dto.response.SessionEvent
import com.skyflydev.threadly.data.sessions.dto.response.SessionPlayerDto
import kotlinx.coroutines.flow.Flow

interface SessionsRepository {
    fun sessionsEventsFlow(): Flow<SessionEvent>
    suspend fun getSessionsSnapshot(): AppResult<List<Session>, AppError>
    suspend fun getSessionDetails(sessionId: String): AppResult<Session, AppError>


    suspend fun startGame(sessionId: String): AppResult<Unit, AppError>
    suspend fun createSession(theme: String, maxRounds: Int): AppResult<SessionCreatedDto, AppError>
    suspend fun joinSession(sessionId: String): AppResult<SessionPlayerDto, AppError>
    suspend fun leaveSession(sessionId: String): AppResult<Unit, AppError>
    suspend fun setReady(value: Boolean, sessionId: String): AppResult<SessionPlayerDto, AppError>
}