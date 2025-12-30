<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import * as pathApi from "@tauri-apps/api/path";
import ControlCenter from './components/ControlCenter.vue';
import apiPost from "./components/apiPost.vue";
const isPanelOpen = ref(false);
const selectedDir = ref<string | null>("");
const inputText = ref("");

async function chooseDir() {
  const result = await open({
    directory: true,
    multiple: false
  });
  if (typeof result === "string") {
    selectedDir.value = result;
  } else if (Array.isArray(result)) {
    selectedDir.value = result[0] ?? null;
  } else {
    selectedDir.value = null;
  }
}

async function saveText() {
  if (!selectedDir.value || !inputText.value) return;
  const filePath = await pathApi.join(selectedDir.value, "output.txt");
  await writeTextFile(filePath, inputText.value);
}
</script>

<template>
  <ControlCenter v-model="isPanelOpen" :maxHeight="550"></ControlCenter>
  <main class="container">
      <div style="margin-bottom: 12px;">
        <apiPost />
        <input v-model="inputText" placeholder="输入文本内容..." style="width: 60%;" />
      </div>
      <div style="margin-bottom: 12px;">
        <button @click="chooseDir">选择目录</button>
        <input :disabled="true"  style="width: 60%;" placeholder="未选择目录..." v-model="selectedDir" />
        <button @click="saveText">保存</button>
        <button @click="selectedDir = ''">清空</button>
      </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 5vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>