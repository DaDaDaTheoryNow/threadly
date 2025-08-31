package com.skyflydev.threadly.core.common.error

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class ApiError(
    val error: ApiErrorContent
)

@Serializable
data class ApiErrorContent(
    val message: String,
    val data: ApiErrorData
)

@Serializable
data class ApiErrorData(
    val detail: String,
    @SerialName("req_uuid")
    val reqUuid: String
)