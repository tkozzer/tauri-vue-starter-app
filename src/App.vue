<script setup>
import { ref, watchEffect, onMounted, onUnmounted } from "vue";
import Greet from "./components/Greet.vue";
import { getCurrentWindow } from '@tauri-apps/api/window';

const isDarkMode = ref(false);
const appWindow = getCurrentWindow();
const isFullscreen = ref(false);
const isSystemTheme = ref(true);
const isToggleVisible = ref(false);

// Function to get the system theme
async function getSystemTheme() {
  const theme = await appWindow.theme();
  return theme === 'dark';
}

// Function to update the app theme
async function updateAppTheme() {
  if (isSystemTheme.value) {
    isDarkMode.value = await getSystemTheme();
  }
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark');
    await appWindow.setTitle("Tauri App (Dark Mode)");
  } else {
    document.documentElement.classList.remove('dark');
    await appWindow.setTitle("Tauri App (Light Mode)");
  }
}

watchEffect(async () => {
  await updateAppTheme();
});

function toggleDarkMode() {
  isSystemTheme.value = false;
  isDarkMode.value = !isDarkMode.value;
}

const preventDefault = (e) => e.preventDefault();

// Add this function to enable window dragging
function startDragging(event) {
  if (event.target.tagName === 'BUTTON') return; // Prevent dragging when clicking buttons
  appWindow.startDragging();
}

// Add these functions to handle window controls
async function closeApp() {
  await appWindow.close();
}

async function minimizeApp() {
  await appWindow.minimize();
}

// Update this function to toggle fullscreen and update isFullscreen state
async function toggleFullscreen() {
  isFullscreen.value = !isFullscreen.value;
  await appWindow.setFullscreen(isFullscreen.value);
}

// Function to check fullscreen state
async function updateFullscreenState() {
  isFullscreen.value = await appWindow.isFullscreen();
}

onMounted(async () => {
  window.addEventListener('wheel', preventDefault, { passive: false });
  window.addEventListener('touchmove', preventDefault, { passive: false });
  await updateFullscreenState();
  // Listen for fullscreen changes
  await appWindow.onResized(updateFullscreenState);

  // Set initial theme
  await updateAppTheme();

  // Listen for system theme changes
  await appWindow.onThemeChanged(async ({ payload: theme }) => {
    if (isSystemTheme.value) {
      isDarkMode.value = theme === 'dark';
      await updateAppTheme();
    }
  });
});

onUnmounted(async () => {
  window.removeEventListener('wheel', preventDefault);
  window.removeEventListener('touchmove', preventDefault);
  // Remove the fullscreen listener
  await appWindow.onResized.removeAll();
});

function showToggle() {
  isToggleVisible.value = true;
}

function hideToggle() {
  isToggleVisible.value = false;
}
</script>

<template>
  <div class="app-container" @mousedown="startDragging">
    <!-- Update the window control buttons to only show when not fullscreen -->
    <div v-if="!isFullscreen" class="window-controls">
      <button @click="closeApp" class="control-button close"></button>
      <button @click="minimizeApp" class="control-button minimize"></button>
      <button @click="toggleFullscreen" class="control-button fullscreen"></button>
    </div>

    <div 
      class="toggle-container"
      @mouseenter="showToggle"
      @mouseleave="hideToggle"
    >
      <button
        @click="toggleDarkMode"
        class="theme-toggle"
        :class="{ 'visible': isToggleVisible }"
      >
        {{ isDarkMode ? '🌞' : '🌙' }}
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
