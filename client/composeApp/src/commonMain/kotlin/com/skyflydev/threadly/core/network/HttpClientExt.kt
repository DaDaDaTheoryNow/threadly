package com.skyflydev.threadly.core.network

import com.skyflydev.threadly.core.common.error.ApiError
import com.skyflydev.threadly.core.common.error.AppError
import com.skyflydev.threadly.core.common.result.AppResult
import io.ktor.client.HttpClient
import io.ktor.client.network.sockets.SocketTimeoutException
import io.ktor.client.plugins.websocket.DefaultClientWebSocketSession
import io.ktor.client.plugins.websocket.WebSocketException
import io.ktor.client.plugins.websocket.webSocket
import io.ktor.client.statement.HttpResponse
import io.ktor.client.statement.bodyAsText
import io.ktor.util.network.UnresolvedAddressException
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.ensureActive
import kotlin.coroutines.coroutineContext

suspend inline fun <reified T> safeCall(
    execute: () -> HttpResponse
): AppResult<T, AppError> {
    val response = try {
        execute()
    } catch (e: SocketTimeoutException) {
        println("safeCall: request timeout")
        return AppResult.Error(AppError.Remote.REQUEST_TIMEOUT)
    } catch (e: UnresolvedAddressException) {
        println("safeCall: no internet")
        return AppResult.Error(AppError.Remote.NO_INTERNET)
    } catch (e: Exception) {
        println("safeCall: unknown error: $e")
        coroutineContext.ensureActive()
        return AppResult.Error(AppError.Remote.UNKNOWN)
    }

    return responseToResult(response)
}

suspend inline fun <reified T> responseToResult(
    response: HttpResponse
): AppResult<T, AppError.Remote> {
    val code = response.status.value
    val text = try {
        response.bodyAsText()
    } catch (e: Exception) {
        println("Failed to read response body: $e")
        "<empty>"
    }

    return if (code in 200..299) {
        if (T::class == Unit::class) {
            return AppResult.Success(Unit as T)
        }

        try {
            val result = JsonConfig.json.decodeFromString<T>(text)
            AppResult.Success(result)
        } catch (e: Exception) {
            println("Serialization error: $e")
            println("Response body: $text")
            AppResult.Error(AppError.Remote.SERIALIZATION)
        }
    } else {
        val apiError = try {
            JsonConfig.json.decodeFromString<ApiError>(text)
        } catch (_: Exception) { null }

        val message = apiError?.error?.message ?: "HTTP $code"
        val detail = apiError?.error?.data?.detail

        val error = if (apiError != null) {
            AppError.Remote.API_ERROR(message = message, detail = detail)
        } else {
            when (code) {
                401 -> AppError.Remote.UNAUTHORIZED
                408 -> AppError.Remote.REQUEST_TIMEOUT
                429 -> AppError.Remote.TOO_MANY_REQUESTS
                in 500..599 -> AppError.Remote.SERVER
                else -> AppError.Remote.UNKNOWN
            }
        }

        println("HTTP error $code, body: $text")
        AppResult.Error(error)
    }
}


suspend fun HttpClient.safeWebSocket(
    urlString: String,
    unauthorizedHandler: UnauthorizedHandler,
    block: suspend DefaultClientWebSocketSession.() -> Unit
): AppResult<Unit, AppError.Remote> = coroutineScope {
    try {
        // Run the webSocket session; this returns Unit
        this@safeWebSocket.webSocket(urlString = urlString, block = block)
        AppResult.Success(Unit)
    } catch (e: WebSocketException) {
        val msg = e.message ?: ""
        println("msg: $msg")
        if (msg.contains("401") || msg.contains("Unauthorized", ignoreCase = true)) {
            try {
                unauthorizedHandler.onUnauthorized()
            } catch (_: Exception) { }
            AppResult.Error(AppError.Remote.UNAUTHORIZED)
        } else {
            AppResult.Error(AppError.Remote.UNKNOWN)
        }
    } catch (e: SocketTimeoutException) {
        AppResult.Error(AppError.Remote.REQUEST_TIMEOUT)
    } catch (e: UnresolvedAddressException) {
        AppResult.Error(AppError.Remote.NO_INTERNET)
    } catch (e: Exception) {
        println("exp: $e")

        coroutineContext.ensureActive()
        AppResult.Error(AppError.Remote.UNKNOWN)
    }
}