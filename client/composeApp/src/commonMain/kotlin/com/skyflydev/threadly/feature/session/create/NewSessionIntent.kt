package com.skyflydev.threadly.feature.session.create_session

sealed class NewSessionIntent {
    data class CreateSession(val theme: String) : NewSessionIntent()
}