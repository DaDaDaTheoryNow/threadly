package com.skyflydev.threadly.data.game.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.model.DomainGameEvent
import kotlinx.coroutines.flow.Flow

interface GameRepository {
    fun gameEventsFlow(sessionId: String): Flow<DomainGameEvent>
    suspend fun submitMessage(sessionId: String, content: String): AppResult<Unit, AppError>
    suspend fun startGame(sessionId: String): AppResult<Unit, AppError>
}