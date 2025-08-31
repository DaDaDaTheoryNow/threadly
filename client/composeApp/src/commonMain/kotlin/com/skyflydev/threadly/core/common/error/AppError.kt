@file:Suppress("ClassName")

package com.skyflydev.threadly.core.common.error

sealed interface AppError : Error {

    sealed interface Remote : AppError {
        object REQUEST_TIMEOUT : Remote
        object TOO_MANY_REQUESTS : Remote
        object NO_INTERNET : Remote
        object SERVER : Remote
        object SERIALIZATION : Remote
        object UNKNOWN : Remote
        object UNAUTHORIZED : Remote
        data class API_ERROR(val message: String, val detail: String?) : Remote
    }

    sealed interface Local : AppError {
        object DISK_FULL : Local
        object UNKNOWN : Local
    }

    data class Unknown(val message: String) : AppError
}

fun AppError.toMessage(): String = when (this) {
    AppError.Remote.REQUEST_TIMEOUT -> "Request timed out"
    AppError.Remote.TOO_MANY_REQUESTS -> "Too many requests"
    AppError.Remote.NO_INTERNET -> "No internet connection"
    AppError.Remote.SERVER -> "Server error"
    AppError.Remote.SERIALIZATION -> "Serialization error"
    AppError.Remote.UNKNOWN -> "Unknown error"
    is AppError.Remote.API_ERROR -> this.detail ?: this.message
    AppError.Local.DISK_FULL -> "Disk full"
    AppError.Local.UNKNOWN -> "Unknown error occurred"
    is AppError.Unknown -> "Unknown error occurred"
    AppError.Remote.UNAUTHORIZED -> "Unauthorized"
}