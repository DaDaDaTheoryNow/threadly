package com.skyflydev.threadly.data.game.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.data.game.dto.GameEvent
import kotlinx.coroutines.flow.Flow

interface GameRepository {
    fun gameEventsFlow(sessionId: String): Flow<GameEvent>
    suspend fun submitMessage(sessionId: String, content: String): AppResult<Unit, AppError>
}