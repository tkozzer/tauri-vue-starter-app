<script setup lang="ts">
import type { Theme } from "@tauri-apps/api/window";

import { listen } from "@tauri-apps/api/event";
import { onMounted, onUnmounted, watchEffect } from "vue";

import Greet from "./components/Greet.vue";
import { useAppStore } from "./stores/appStore";

const store = useAppStore();

watchEffect(async () => {
  await store.updateAppTheme();
});

const preventDefault = (e: Event): void => e.preventDefault();

// Add this function to enable window dragging
function startDragging(event: MouseEvent): void {
  const target = event.target as HTMLElement;
  if (target.tagName === "BUTTON")
    return; // Prevent dragging when clicking buttons
  store.appWindow.startDragging();
}

// Add these functions to handle window controls
async function closeApp(): Promise<void> {
  await store.appWindow.close();
}

async function minimizeApp(): Promise<void> {
  await store.appWindow.minimize();
}

// Update this function to toggle fullscreen and update isFullscreen state
async function toggleFullscreen(): Promise<void> {
  store.isFullscreen = !store.isFullscreen;
  await store.appWindow.setFullscreen(store.isFullscreen);
}

// Function to check fullscreen state
async function updateFullscreenState(): Promise<void> {
  store.isFullscreen = await store.appWindow.isFullscreen();
}

onMounted(async () => {
  window.addEventListener("wheel", preventDefault, { passive: false });
  window.addEventListener("touchmove", preventDefault, { passive: false });
  await updateFullscreenState();

  // Listen for fullscreen changes and emit window data
  await store.appWindow.onResized(async () => {
    await updateFullscreenState();
    await store.emitWindowData();
  });

  // Listen for window moves and emit window data
  await store.appWindow.onMoved(store.emitWindowData);

  // Set initial theme
  await store.updateAppTheme();

  // Listen for system theme changes
  await store.appWindow.onThemeChanged(async (event: { payload: Theme }) => {
    if (store.isSystemTheme) {
      store.isDarkMode = event.payload === "dark";
      await store.updateAppTheme();
    }
  });

  // Emit initial window data
  await store.emitWindowData();

  // Listen for requests from debug window
  await listen("request-main-window-info", store.emitWindowData);
});

onUnmounted(() => {
  window.removeEventListener("wheel", preventDefault);
  window.removeEventListener("touchmove", preventDefault);
  // Note: Event listeners will be automatically cleaned up when the component unmounts
});
</script>

<template>
  <div class="app-container" @mousedown="startDragging">
    <!-- Update the window control buttons to only show when not fullscreen -->
    <div v-if="!store.isFullscreen" class="window-controls">
      <button class="control-button close" @click="closeApp" />
      <button class="control-button minimize" @click="minimizeApp" />
      <button class="control-button fullscreen" @click="toggleFullscreen" />
    </div>

    <div
      class="toggle-container"
      @mouseenter="store.showToggle"
      @mouseleave="store.hideToggle"
    >
      <button
        class="theme-toggle"
        :class="{ visible: store.isToggleVisible }"
        @click="store.toggleDarkMode"
      >
        {{ store.isDarkMode ? '🌞' : '🌙' }}
      </button>
    </div>

    <h1 class="text-4xl font-bold mb-8">
      Welcome to Tauri!
    </h1>

    <div class="flex justify-center space-x-4 mb-8">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo h-16" alt="Vite logo">
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo h-16" alt="Tauri logo">
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo h-16" alt="Vue logo">
      </a>
    </div>

    <p class="mb-8">
      Click on the Tauri, Vite, and Vue logos to learn more.
    </p>

    <Greet />
  </div>
</template>

<style>
@reference "./index.css";

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
