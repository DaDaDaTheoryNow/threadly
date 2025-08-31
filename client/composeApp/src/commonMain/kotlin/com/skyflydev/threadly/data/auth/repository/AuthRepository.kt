package com.skyflydev.threadly.data.auth.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import kotlinx.coroutines.flow.Flow

interface AuthRepository {
    val isUserLoggedIn: Flow<Boolean>

    suspend fun signIn(username: String, password: String) : AppResult<String, AppError>
    suspend fun signUp(username: String, password: String) : AppResult<String, AppError>
    suspend fun signOut() : AppResult<Unit, AppError>
}