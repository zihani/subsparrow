<template>
  <div class="response-info-display">
    <div class="info">
      <div>状态：<code>{{ responseStatus ?? '-' }}</code></div>
      <div>耗时：<code>{{ responseTime ?? '-' }} ms</code></div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  responseStatus: number | null;
  responseTime: number | null;
}>();
</script>

<style scoped>
.response-info-display {
  width: 100%;
}
.info {
  display: grid;
  gap: 6px;
}
.info > div {
  padding: 4px 0;
}
.info code {
  background: #f9fafc;
  padding: 2px 4px;
  border-radius: 3px;
}
</style>