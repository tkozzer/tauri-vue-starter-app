<script setup>
import { ref, watchEffect, onMounted, onUnmounted } from "vue";
import Greet from "./components/Greet.vue";
import { getCurrentWindow } from '@tauri-apps/api/window';

const isDarkMode = ref(false);
const appWindow = getCurrentWindow();

watchEffect(async () => {
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark');
    await appWindow.setTitle("Tauri App (Dark Mode)");
  } else {
    document.documentElement.classList.remove('dark');
    await appWindow.setTitle("Tauri App (Light Mode)");
  }
});

function toggleDarkMode() {
  isDarkMode.value = !isDarkMode.value;
}

const preventDefault = (e) => e.preventDefault();

onMounted(() => {
  window.addEventListener('wheel', preventDefault, { passive: false });
  window.addEventListener('touchmove', preventDefault, { passive: false });
});

onUnmounted(() => {
  window.removeEventListener('wheel', preventDefault);
  window.removeEventListener('touchmove', preventDefault);
});
</script>

<template>
  <div class="app-container">
    <button
      @click="toggleDarkMode"
      class="absolute top-4 right-4 p-2 rounded-full bg-gray-200 dark:bg-gray-700"
    >
      {{ isDarkMode ? '🌞' : '🌙' }}
    </button>

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
</style>
