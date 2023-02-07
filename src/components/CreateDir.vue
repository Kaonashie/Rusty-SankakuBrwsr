<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const resMsg = ref("")
const path = ref("")

async function createDir() {
    resMsg.value = await invoke("create_dir", { path: path.value })
}
</script>

<template>
  <div>
      <input id="path-input" v-model="path" placeholder="Enter a path..." />
      <button type="button" @click="createDir()">Create directory!</button>
  </div>
  <p>{{ resMsg }}</p>
</template>