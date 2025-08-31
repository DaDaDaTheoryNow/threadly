package com.skyflydev.threadly.data.game.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.model.DomainGameEvent
import com.skyflydev.threadly.data.game.datasources.network.GameNetworkDataSource
import com.skyflydev.threadly.data.game.mappers.toDomain
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class GameRepositoryImpl(
    private val network: GameNetworkDataSource,
    ) : GameRepository {
    override fun gameEventsFlow(sessionId: String): Flow<DomainGameEvent> {
        return network.observeGameEvents(sessionId).map { it.toDomain() }
    }

    override suspend fun submitMessage(
        sessionId: String,
        content: String,
    ): AppResult<Unit, AppError> {
        return network.submitMessage(sessionId, content)
    }

    override suspend fun startGame(sessionId: String): AppResult<Unit, AppError> {
        return network.startGame(sessionId)
    }
}