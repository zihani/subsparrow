<template>
  <div class="response-info-display">
    <div class="info">
      <div>状态：{{ responseStatus ?? '-' }}</div>
      <div>耗时：{{ responseTime ?? '-' }} ms</div>
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
</style>