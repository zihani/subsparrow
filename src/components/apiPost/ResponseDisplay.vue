<template>
  <div class="response-display">
    <pre class="response-pre">{{ prettyJson(responseData) }}</pre>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  responseData: any;
}>();

const prettyJson = (data: any) => {
  try {
    return typeof data === 'string' ? data : JSON.stringify(data, null, 2);
  } catch {
    return String(data);
  }
};
</script>

<style scoped>
.response-display {
  width: 100%;
}
.response-pre {
  padding: 8px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #f9fafc;
  white-space: pre-wrap;
  word-wrap: break-word;
  margin: 0;
}
</style>