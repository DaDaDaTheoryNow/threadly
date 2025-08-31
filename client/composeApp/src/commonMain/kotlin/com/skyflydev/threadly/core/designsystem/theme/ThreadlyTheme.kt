package com.skyflydev.threadly.core.designsystem.theme

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Shapes
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.unit.dp

val DarkColorTheme = darkColorScheme(
    primary = AccentWhite,        // Основной акцент — белый
    onPrimary = OnPrimary,        // Черный текст на белой кнопке
    secondary = MediumWhite,      // Вторичный цвет
    onSecondary = NearBlack,
    background = DarkBackground,  // Фон
    onBackground = OnBackground,  // Текст на фоне
    surface = DarkSurface,        // Поверхность карточек
    onSurface = OnSurface,        // Текст на карточках
    surfaceVariant = DarkSurfaceVariant,
    onSurfaceVariant = OnSurfaceVariant,
    error = ErrorRed,
    onError = OnError
)

val Shapes = Shapes(
    extraSmall = RoundedCornerShape(4.dp),
    small = RoundedCornerShape(8.dp),
    medium = RoundedCornerShape(12.dp),
    large = RoundedCornerShape(20.dp),
    extraLarge = RoundedCornerShape(28.dp)
)

@Composable
fun ThreadlyTheme(
    content: @Composable () -> Unit,
) {
    val theme = DarkColorTheme

    MaterialTheme(
        colorScheme = theme,
        content = content,
        shapes = Shapes
    )
}
