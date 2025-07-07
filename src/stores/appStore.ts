import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getCurrentWindow, Window } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'
import { PhysicalPosition, PhysicalSize } from '@tauri-apps/api/dpi'

interface WindowInfo {
    position: PhysicalPosition
    size: PhysicalSize
}

export const useAppStore = defineStore('app', () => {
    const isDarkMode = ref<boolean>(false)
    const isFullscreen = ref<boolean>(false)
    const isSystemTheme = ref<boolean>(true)
    const isToggleVisible = ref<boolean>(false)
    const appWindow: Window = getCurrentWindow()

    const getSystemTheme = async (): Promise<boolean> => {
        const theme = await appWindow.theme()
        return theme === 'dark'
    }

    const updateAppTheme = async (): Promise<void> => {
        if (isSystemTheme.value) {
            isDarkMode.value = await getSystemTheme()
        }
        if (isDarkMode.value) {
            document.documentElement.classList.add('dark')
            await appWindow.setTitle("Tauri App (Dark Mode)")
        } else {
            document.documentElement.classList.remove('dark')
            await appWindow.setTitle("Tauri App (Light Mode)")
        }
    }

    const toggleDarkMode = (): void => {
        isSystemTheme.value = false
        isDarkMode.value = !isDarkMode.value
    }

    const showToggle = (): void => {
        isToggleVisible.value = true
    }

    const hideToggle = (): void => {
        isToggleVisible.value = false
    }

    const emitWindowData = async (): Promise<void> => {
        const position = await appWindow.innerPosition()
        const size = await appWindow.innerSize()
        const windowInfo: WindowInfo = { position, size }
        await emit('main-window-info', windowInfo)
    }

    return {
        isDarkMode,
        isFullscreen,
        isSystemTheme,
        isToggleVisible,
        appWindow,
        getSystemTheme,
        updateAppTheme,
        toggleDarkMode,
        showToggle,
        hideToggle,
        emitWindowData,
    }
}) 