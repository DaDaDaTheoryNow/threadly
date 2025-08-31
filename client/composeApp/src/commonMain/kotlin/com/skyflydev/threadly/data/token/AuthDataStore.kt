package com.skyflydev.threadly.data.token

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.model.AuthData

interface AuthDataStore {
    suspend fun saveAuthData(authData: AuthData): AppResult<Unit, AppError>
    suspend fun getAuthData(): AppResult<AuthData, AppError>
    suspend fun removeAuthData(): AppResult<Unit, AppError>
}