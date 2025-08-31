package com.skyflydev.threadly.di

import com.russhwolf.settings.Settings
import com.russhwolf.settings.StorageSettings
import io.ktor.client.engine.HttpClientEngine
import io.ktor.client.engine.js.Js
import org.koin.core.module.Module
import org.koin.dsl.module

actual val platformModule: Module
    get() = module {
        single<HttpClientEngine> { Js.create() }
        single<Settings> { StorageSettings() }
    }