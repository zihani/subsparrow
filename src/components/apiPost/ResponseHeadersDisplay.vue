<template>
  <div class="response-headers-display">
    <div class="table">
      <div class="table-head">
        <span>Header</span><span>Value</span>
      </div>
      <div class="table-row" v-for="(v,k) in responseHeaders" :key="k">
        <span>{{ k }}</span><span>{{ v }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  responseHeaders: Record<string, string>;
}>();
</script>

<style scoped>
.response-headers-display {
  width: 100%;
}
.table {
  display: grid;
  gap: 8px;
}
.table-head, .table-row {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 8px;
  align-items: center;
  padding: 4px 0;
}
.table-head {
  font-weight: bold;
  border-bottom: 1px solid #dcdfe6;
}
</style>