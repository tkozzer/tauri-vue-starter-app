<script setup>
import { watchEffect, onMounted, onUnmounted } from "vue"
import { useAppStore } from "./stores/appStore"
import Greet from "./components/Greet.vue"

const store = useAppStore()

watchEffect(async () => {
  await store.updateAppTheme()
})

const preventDefault = (e) => e.preventDefault()

// Add this function to enable window dragging
function startDragging(event) {
  if (event.target.tagName === 'BUTTON') return // Prevent dragging when clicking buttons
  store.appWindow.startDragging()
}

// Add these functions to handle window controls
async function closeApp() {
  await store.appWindow.close()
}

async function minimizeApp() {
  await store.appWindow.minimize()
}

// Update this function to toggle fullscreen and update isFullscreen state
async function toggleFullscreen() {
  store.isFullscreen = !store.isFullscreen
  await store.appWindow.setFullscreen(store.isFullscreen)
}

// Function to check fullscreen state
async function updateFullscreenState() {
  store.isFullscreen = await store.appWindow.isFullscreen()
}

onMounted(async () => {
  window.addEventListener('wheel', preventDefault, { passive: false })
  window.addEventListener('touchmove', preventDefault, { passive: false })
  await updateFullscreenState()
  // Listen for fullscreen changes
  await store.appWindow.onResized(updateFullscreenState)

  // Set initial theme
  await store.updateAppTheme()

  // Listen for system theme changes
  await store.appWindow.onThemeChanged(async ({ payload: theme }) => {
    if (store.isSystemTheme) {
      store.isDarkMode = theme === 'dark'
      await store.updateAppTheme()
    }
  })
})

onUnmounted(async () => {
  window.removeEventListener('wheel', preventDefault)
  window.removeEventListener('touchmove', preventDefault)
  // Remove the fullscreen listener
  await store.appWindow.onResized.removeAll()
})
</script>

<template>
  <div class="app-container" @mousedown="startDragging">
    <!-- Update the window control buttons to only show when not fullscreen -->
    <div v-if="!store.isFullscreen" class="window-controls">
      <button @click="closeApp" class="control-button close"></button>
      <button @click="minimizeApp" class="control-button minimize"></button>
      <button @click="toggleFullscreen" class="control-button fullscreen"></button>
    </div>

    <div 
      class="toggle-container"
      @mouseenter="store.showToggle"
      @mouseleave="store.hideToggle"
    >
      <button
        @click="store.toggleDarkMode"
        class="theme-toggle"
        :class="{ 'visible': store.isToggleVisible }"
      >
        {{ store.isDarkMode ? '🌞' : '🌙' }}
      </button>
    </div>

    <h1 class="text-4xl font-bold mb-8">Welcome to Tauri!</h1>

    <div class="flex justify-center space-x-4 mb-8">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo h-16" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo h-16" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo h-16" alt="Vue logo" />
      </a>
    </div>

    <p class="mb-8">Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <Greet />
  </div>
</template>

<style>
.app-container {
  @apply min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100;
  height: 100vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
  padding: 8px; /* Add padding to prevent content from touching the edges */
}

/* To ensure child elements don't overflow the rounded corners */
.app-container > * {
  max-width: 100%;
  max-height: 100%;
}

.logo {
  transition: filter 0.75s;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

/* Update the window controls style to be more visible when not in fullscreen */
.window-controls {
  position: absolute;
  top: 8px;
  left: 8px;
  display: flex;
  gap: 8px;
  z-index: 1000; /* Ensure it's above other elements */
}

.control-button {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
}

.close {
  background-color: #ff5f56;
}

.minimize {
  background-color: #ffbd2e;
}

.fullscreen {
  background-color: #27c93f;
}

.maximize {
  background-color: #27c93f;
}

.toggle-container {
  position: absolute;
  top: 0;
  right: 0;
  padding: 16px;
  z-index: 10;
}

.theme-toggle {
  opacity: 0;
  transition: opacity 0.3s ease-in-out;
  @apply p-2 rounded-full bg-gray-200 dark:bg-gray-700;
}

.theme-toggle.visible {
  opacity: 1;
}
</style>
