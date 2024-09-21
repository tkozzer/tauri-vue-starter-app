<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <form class="flex items-center justify-center mb-4" @submit.prevent="greet">
    <input
      id="greet-input"
      v-model="name"
      placeholder="Enter a name..."
      class="mr-2 px-4 py-2 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 border border-gray-300 dark:border-gray-700"
    />
    <button
      type="submit"
      class="px-4 py-2 rounded bg-blue-500 text-white hover:bg-blue-600 transition-colors"
    >
      Greet
    </button>
  </form>

  <p class="text-lg">{{ greetMsg }}</p>
</template>
