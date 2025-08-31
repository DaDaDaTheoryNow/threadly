package com.skyflydev.threadly.data.auth.repository

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.common.result.map
import com.skyflydev.threadly.core.common.result.onSuccess
import com.skyflydev.threadly.data.auth.datasources.network.AuthNetworkDataSource
import com.skyflydev.threadly.data.auth.mappers.toAuthData
import com.skyflydev.threadly.data.token.AuthDataStore
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class AuthRepositoryImpl(
    private val localDataSource: AuthDataStore,
    private val remoteDataSource: AuthNetworkDataSource
) : AuthRepository {
    private val _isUserLoggedIn = MutableStateFlow<Boolean>(false)
    override val isUserLoggedIn = _isUserLoggedIn.asStateFlow()

    private val scope = CoroutineScope(Dispatchers.Default)

    init {
        scope.launch {
            localDataSource.getAuthData().onSuccess {
                _isUserLoggedIn.value = true
            }
        }
    }

    override suspend fun signIn(
        username: String,
        password: String
    ): AppResult<String, AppError> {
        return remoteDataSource
            .signIn(username, password)
            .onSuccess { resp ->
                localDataSource.saveAuthData(resp.toAuthData())
                _isUserLoggedIn.value = true
            }
            .map {
                it.token
            }
    }

    override suspend fun signUp(
        username: String,
        password: String
    ): AppResult<String, AppError> {
        return remoteDataSource
            .signUp(username, password)
            .onSuccess { resp ->
                localDataSource.saveAuthData(resp.toAuthData())
                _isUserLoggedIn.value = true
            }
            .map {
                it.token
            }
    }

    override suspend fun signOut(): AppResult<Unit, AppError> {
        _isUserLoggedIn.value = false
        return localDataSource.removeAuthData()
    }
}