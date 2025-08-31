package com.skyflydev.threadly.core.network

/**
 * Functional interface invoked when an unauthorized (401) situation is detected.
 * Implementations typically perform sign-out/cleanup and navigation to auth screen.
 */
fun interface UnauthorizedHandler {
    suspend fun onUnauthorized()
}