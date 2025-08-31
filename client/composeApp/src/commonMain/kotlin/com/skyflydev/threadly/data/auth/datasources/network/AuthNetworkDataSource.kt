package com.skyflydev.threadly.data.auth.datasources.network

import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import com.skyflydev.threadly.core.network.safeCall
import com.skyflydev.threadly.data.auth.dto.AuthResponseDto
import io.ktor.client.HttpClient
import io.ktor.client.request.post
import io.ktor.client.request.setBody

interface AuthNetworkDataSource {
    suspend fun signIn(username: String, password: String) : AppResult<AuthResponseDto, AppError>
    suspend fun signUp(username: String, password: String) : AppResult<AuthResponseDto, AppError>
}

class AuthNetworkDataSourceImpl(
    private val httpClient: HttpClient
) : AuthNetworkDataSource {
    override suspend fun signIn(username: String, password: String): AppResult<AuthResponseDto, AppError> {
        return safeCall<AuthResponseDto> {
            httpClient.post(
                urlString = "/login"
            ) {
                setBody(
                    mapOf(
                        "username" to username,
                        "password" to password
                    )
                )
            }
        }
    }

    override suspend fun signUp(
        username: String,
        password: String
    ): AppResult<AuthResponseDto, AppError> {
        return safeCall<AuthResponseDto> {
            httpClient.post(
                urlString = "/register"
            ) {
                setBody(
                    mapOf(
                        "username" to username,
                        "password" to password
                    )
                )
            }
        }
    }
}

