package com.skyflydev.threadly.data.token

import com.russhwolf.settings.Settings
import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.model.AuthData

class LocalAuthDataStore(
    private val settings: Settings
) : AuthDataStore {
    override suspend fun saveAuthData(authData: AuthData): AppResult<Unit, AppError> {
        return try {
            settings.putString(TOKEN_KEY, authData.token)
            settings.putString(USER_ID_KEY, authData.userId)
            AppResult.Success(Unit)
        } catch (e: Exception) {
            AppResult.Error(AppError.Unknown(e.message ?: "Failed to save token"))
        }
    }

    override suspend fun getAuthData(): AppResult<AuthData, AppError> {
        return try {
            val token = settings.getStringOrNull(TOKEN_KEY)
            val userId = settings.getStringOrNull(USER_ID_KEY)
            if (token != null && userId != null) {
                AppResult.Success(AuthData(token, userId))
            } else {
                AppResult.Error(AppError.Unknown("Token not found"))
            }
        } catch (e: Exception) {
            AppResult.Error(AppError.Unknown(e.message ?: "Failed to get token"))
        }
    }

    override suspend fun removeAuthData(): AppResult<Unit, AppError> {
        return try {
            settings.remove(TOKEN_KEY)
            settings.remove(USER_ID_KEY)
            AppResult.Success(Unit)
        } catch (e: Exception) {
            AppResult.Error(AppError.Unknown(e.message ?: "Failed to remove token"))
        }
    }

    companion object {
        const val TOKEN_KEY = "auth_token"
        const val USER_ID_KEY = "user_id"
    }
}