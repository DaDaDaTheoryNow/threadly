@file:Suppress("unused")

package com.skyflydev.threadly.core.common.result

import com.skyflydev.threadly.core.common.error.Error

sealed interface AppResult<out D, out E: Error> {
    data class Success<out D>(val data: D): AppResult<D, Nothing>
    data class Error<out E: com.skyflydev.threadly.core.common.error.Error>(val error: E):
        AppResult<Nothing, E>
}

inline fun <T, E: Error, R> AppResult<T, E>.map(map: (T) -> R): AppResult<R, E> {
    return when(this) {
        is AppResult.Error -> AppResult.Error(error)
        is AppResult.Success -> AppResult.Success(map(data))
    }
}

fun <T, E: Error> AppResult<T, E>.asEmptyDataResult(): EmptyResult<E> {
    return map {  }
}

inline fun <T, E: Error> AppResult<T, E>.onSuccess(action: (T) -> Unit): AppResult<T, E> {
    return when(this) {
        is AppResult.Error -> this
        is AppResult.Success -> {
            action(data)
            this
        }
    }
}
inline fun <T, E: Error> AppResult<T, E>.onError(action: (E) -> Unit): AppResult<T, E> {
    return when(this) {
        is AppResult.Error -> {
            action(error)
            this
        }
        is AppResult.Success -> this
    }
}

typealias EmptyResult<E> = AppResult<Unit, E>