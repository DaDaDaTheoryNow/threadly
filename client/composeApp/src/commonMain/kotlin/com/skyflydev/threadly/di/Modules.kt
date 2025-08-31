package com.skyflydev.threadly.di

import com.skyflydev.threadly.core.network.HttpClientFactory
import com.skyflydev.threadly.core.network.UnauthorizedHandler
import com.skyflydev.threadly.data.auth.datasources.network.AuthNetworkDataSource
import com.skyflydev.threadly.data.auth.datasources.network.AuthNetworkDataSourceImpl
import com.skyflydev.threadly.data.auth.repository.AuthRepository
import com.skyflydev.threadly.data.auth.repository.AuthRepositoryImpl
import com.skyflydev.threadly.data.game.datasources.network.GameNetworkDataSource
import com.skyflydev.threadly.data.game.datasources.network.GameNetworkDataSourceImpl
import com.skyflydev.threadly.data.game.repository.GameRepository
import com.skyflydev.threadly.data.game.repository.GameRepositoryImpl
import com.skyflydev.threadly.data.sessions.datasources.network.SessionsNetworkDataSource
import com.skyflydev.threadly.data.sessions.datasources.network.SessionsNetworkDataSourceImpl
import com.skyflydev.threadly.data.sessions.repository.SessionsRepository
import com.skyflydev.threadly.data.sessions.repository.SessionsRepositoryImpl
import com.skyflydev.threadly.data.token.AuthDataStore
import com.skyflydev.threadly.data.token.LocalAuthDataStore
import com.skyflydev.threadly.feature.auth.view_model.AuthViewModel
import com.skyflydev.threadly.feature.game.view_model.GameViewModel
import com.skyflydev.threadly.feature.home.view_model.HomeViewModel
import com.skyflydev.threadly.feature.session.create.NewSessionViewModel
import com.skyflydev.threadly.feature.session.lobby.view_model.SessionLobbyViewModel
import org.koin.core.module.Module
import org.koin.core.module.dsl.singleOf
import org.koin.core.module.dsl.viewModel
import org.koin.core.module.dsl.viewModelOf
import org.koin.dsl.bind
import org.koin.dsl.module

expect val platformModule: Module

val sharedModule = module {
    single<UnauthorizedHandler> { UnauthorizedHandler { get<AuthRepository>().signOut() } }

    // HttpClient
    single {
        HttpClientFactory.create(
            engine = get(),
            unauthorizedHandler = get()
        )
    }

    // Token Section
    singleOf(::LocalAuthDataStore).bind<AuthDataStore>()

    // Auth Section
    singleOf(::AuthNetworkDataSourceImpl).bind<AuthNetworkDataSource>()
    singleOf(::AuthRepositoryImpl).bind<AuthRepository>()

    // Sessions Section
    singleOf(::SessionsNetworkDataSourceImpl).bind<SessionsNetworkDataSource>()
    singleOf(::SessionsRepositoryImpl).bind<SessionsRepository>()

    // Game Section
    singleOf(::GameNetworkDataSourceImpl).bind<GameNetworkDataSource>()
    singleOf(::GameRepositoryImpl).bind<GameRepository>()

    // ViewModels
    viewModelOf(::HomeViewModel)
    viewModelOf(::AuthViewModel)
    viewModel { NewSessionViewModel(get(), get()) }
    viewModel { (sessionId: String, userId: String) ->
        SessionLobbyViewModel(sessionId, userId, get())
    }
    viewModel { (sessionId: String, userId: String, isHost: Boolean) ->
        GameViewModel(sessionId, userId, isHost, get())
    }
}