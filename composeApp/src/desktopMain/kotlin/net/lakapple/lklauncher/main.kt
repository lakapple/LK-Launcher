package net.lakapple.lklauncher

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application

fun main() = application {
    Window(
        onCloseRequest = ::exitApplication,
        title = "LK Launcher",
    ) {
        App()
    }
}