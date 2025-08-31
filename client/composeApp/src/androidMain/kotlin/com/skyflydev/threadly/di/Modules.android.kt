package com.skyflydev.threadly.di

import android.content.Context
import com.russhwolf.settings.Settings
import com.russhwolf.settings.SharedPreferencesSettings
import io.ktor.client.engine.HttpClientEngine
import io.ktor.client.engine.okhttp.OkHttp
import org.koin.android.ext.koin.androidContext
import org.koin.core.module.Module
import org.koin.dsl.module

actual val platformModule: Module
    get() = module {
        single<HttpClientEngine> { OkHttp.create() }
        single<Settings> { SharedPreferencesSettings(
            delegate = androidContext().getSharedPreferences("threadly_prefs", Context.MODE_PRIVATE),
            commit = true
        ) }
    }