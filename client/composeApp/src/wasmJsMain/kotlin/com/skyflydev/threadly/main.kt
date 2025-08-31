package com.skyflydev.threadly

import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.window.ComposeViewport
import com.skyflydev.threadly.di.initKoin
import kotlinx.browser.document


@OptIn(ExperimentalComposeUiApi::class)
fun main() {
    initKoin { }

    ComposeViewport(document.body!!) {
        App()
    }
}