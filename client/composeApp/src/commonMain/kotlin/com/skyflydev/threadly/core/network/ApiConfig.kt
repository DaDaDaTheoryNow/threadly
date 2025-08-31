package com.skyflydev.threadly.core.network

object ApiConfig {
    const val HOST = "192.168.0.101"
//    const val HOST = "localhost"
    const val PORT = 3000

    const val BASE_HTTP_URL = "http://$HOST:$PORT"
    const val BASE_WS_URL = "ws://$HOST:$PORT"
}
