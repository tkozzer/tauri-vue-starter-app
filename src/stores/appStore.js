import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'

export const useAppStore = defineStore('app', () => {
    const isDarkMode = ref(false)
    const isFullscreen = ref(false)
    const isSystemTheme = ref(true)
    const isToggleVisible = ref(false)
    const appWindow = getCurrentWindow()

    const getSystemTheme = async () => {
        const theme = await appWindow.theme()
        return theme === 'dark'
    }

    const updateAppTheme = async () => {
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

    const toggleDarkMode = () => {
        isSystemTheme.value = false
        isDarkMode.value = !isDarkMode.value
    }

    const showToggle = () => {
        isToggleVisible.value = true
    }

    const hideToggle = () => {
        isToggleVisible.value = false
    }

    const emitWindowData = async () => {
        const position = await appWindow.innerPosition()
        const size = await appWindow.innerSize()
        await emit('main-window-info', { position, size })
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