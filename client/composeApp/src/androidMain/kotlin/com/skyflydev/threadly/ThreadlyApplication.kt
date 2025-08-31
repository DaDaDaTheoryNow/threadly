package com.skyflydev.threadly

import android.app.Application
import com.skyflydev.threadly.di.initKoin
import org.koin.android.ext.koin.androidContext

class ThreadlyApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        initKoin {
            androidContext(this@ThreadlyApplication)
        }
    }
}