package com.skyflydev.threadly.data.auth.mappers

import com.skyflydev.threadly.core.model.AuthData
import com.skyflydev.threadly.data.auth.dto.AuthResponseDto

fun AuthResponseDto.toAuthData(): AuthData {
    return AuthData(
        token = token,
        userId = userId
    )
}