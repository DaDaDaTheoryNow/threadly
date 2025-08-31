package com.skyflydev.threadly.core.network

import io.ktor.client.HttpClient
import io.ktor.client.engine.HttpClientEngine
import io.ktor.client.plugins.HttpResponseValidator
import io.ktor.client.plugins.HttpTimeout
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.plugins.defaultRequest
import io.ktor.client.plugins.logging.LogLevel
import io.ktor.client.plugins.logging.Logger
import io.ktor.client.plugins.logging.Logging
import io.ktor.client.plugins.websocket.WebSockets
import io.ktor.http.ContentType
import io.ktor.http.HttpStatusCode
import io.ktor.http.buildUrl
import io.ktor.http.contentType
import io.ktor.http.takeFrom
import io.ktor.serialization.kotlinx.json.json
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.serialization.ExperimentalSerializationApi

object HttpClientFactory {
    @OptIn(ExperimentalSerializationApi::class)
    fun create(
        engine: HttpClientEngine,
        unauthorizedHandler: UnauthorizedHandler
    ): HttpClient {
        return HttpClient(engine) {
            defaultRequest {
                contentType(ContentType.Application.Json)
                url { takeFrom(ApiConfig.BASE_HTTP_URL) }
            }

            HttpResponseValidator {
                validateResponse { response ->
                    if (response.status == HttpStatusCode.Unauthorized) {
                        CoroutineScope(Dispatchers.Default).launch {
                            unauthorizedHandler.onUnauthorized()
                        }
                     }
                }
            }

            install(WebSockets) {
                buildUrl {
                    takeFrom(ApiConfig.BASE_WS_URL)
                }
            }

            install(ContentNegotiation) {
                json(
                    json = JsonConfig.json
                )
            }

            install(HttpTimeout) {
                socketTimeoutMillis = 20_000L
                requestTimeoutMillis = 20_000L
            }

            install(Logging) {
                logger = object : Logger {
                    override fun log(message: String) {
                        println(message)
                    }
                }
                level = LogLevel.ALL
            }
        }
    }
}