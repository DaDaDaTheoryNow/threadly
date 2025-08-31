package com.skyflydev.threadly.core.designsystem.theme

import androidx.compose.ui.graphics.Color

// Новая монохромная палитра с акцентом на контраст
val TrueBlack = Color(0xFF000000)         // Чистый черный
val NearBlack = Color(0xFF121212)         // Почти черный (основной фон)
val DarkGray = Color(0xFF1E1E1E)          // Для поверхностей
val MediumGray = Color(0xFF2D2D2D)        // Для вариаций поверхностей, делимитеров

// Белые и серые оттенки для текста и иконок
val TrueWhite = Color(0xFFFFFFFF)         // Чистый белый (для максимального контраста)
val BrightWhite = Color(0xFFF5F5F5)       // Основной текст
val MediumWhite = Color(0xFFE0E0E0)       // Вторичный текст
val DimWhite = Color(0xFFBDBDBD)          // Неактивные элементы, хинты

// Акценты (оставим ТОЛЬКО один, монохромный, но можно выбрать)
// Вариант 1: Классический белый акцент (рекомендуется)
val AccentWhite = Color(0xFFFFFFFF)
// Вариант 2: Светло-серый акцент для более мягкого вида
// val AccentGray = Color(0xFFCCCCCC)

// Статусные цвета в монохромной стилистике
val ErrorRed = Color(0xFFCF6679)          // Можно оставить, он хорошо работает на темном фоне
val SuccessWhite = Color(0xFFE0E0E0)      // Успех просто подсвечивается белым

// Фоны и поверхности (переопределяем)
val DarkBackground = NearBlack            // Глубокий фон
val DarkSurface = DarkGray                // Для карточек и панелей
val DarkSurfaceVariant = MediumGray       // Вариации

// Текст и элементы (переопределяем)
val OnPrimary = TrueBlack                 // На акцентном цвете (если он белый)
val OnBackground = BrightWhite            // Основной текст на фоне
val OnSurface = MediumWhite               // Текст на карточках
val OnSurfaceVariant = DimWhite           // Вторичный, менее важный текст
val OnError = TrueWhite                   // Текст на ошибке