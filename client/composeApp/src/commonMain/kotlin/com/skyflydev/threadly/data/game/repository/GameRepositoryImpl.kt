package com.skyflydev.threadly.data.game.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.data.game.datasources.network.GameNetworkDataSource
import com.skyflydev.threadly.data.game.dto.GameEvent
import kotlinx.coroutines.flow.Flow

class GameRepositoryImpl(
    private val network: GameNetworkDataSource,
    ) : GameRepository {
    override fun gameEventsFlow(sessionId: String): Flow<GameEvent> {
        return network.observeGameEvents(sessionId)
    }

    override suspend fun submitMessage(
        sessionId: String,
        content: String,
    ): AppResult<Unit, AppError> {
        return network.submitMessage(sessionId, content)
    }
}