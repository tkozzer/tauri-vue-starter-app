<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

import { Button as UiButton } from "@/components/ui/button";
import { Input as UiInput } from "@/components/ui/input";

const greetMsg = ref<string>("");
const name = ref<string>("");

async function greet(): Promise<void> {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke<string>("greet", { name: name.value });
}
</script>

<template>
  <form class="flex items-center justify-center mb-4" @submit.prevent="greet">
    <UiInput
      id="greet-input"
      v-model="name"
      placeholder="Enter a name..."
      class="mr-2"
    />
    <UiButton type="submit">
      Greet
    </UiButton>
  </form>

  <p class="text-lg">
    {{ greetMsg }}
  </p>
</template>
