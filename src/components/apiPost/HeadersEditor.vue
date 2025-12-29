<template>
  <div class="headers-editor">
    <div class="table">
      <div class="table-head">
        <span>Key</span><span>Value</span><span>Description</span><span>操作</span>
      </div>
      <div class="table-row" v-for="(row, i) in localHeaders" :key="'h-'+i">
        <input v-model="row.key" placeholder="Header name" />
        <input v-model="row.value" placeholder="Header value" />
        <input v-model="row.description" placeholder="description" />
        <button @click="removeRow(i)">删除</button>
      </div>
      <div class="table-actions">
        <button @click="addRow">新增一行</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

interface HeaderRow {
  key: string;
  value: string;
  description?: string;
}

const props = defineProps<{
  headers: HeaderRow[];
}>();

const emit = defineEmits(['update:headers']);

const localHeaders = ref<HeaderRow[]>(props.headers);

watch(localHeaders, (newVal) => {
  emit('update:headers', newVal);
}, { deep: true });

const addRow = () => {
  localHeaders.value.push({ key: '', value: '', description: '' });
};

const removeRow = (index: number) => {
  localHeaders.value.splice(index, 1);
};
</script>

<style scoped>
.headers-editor {
  width: 100%;
}
.table {
  display: grid;
  gap: 8px;
}
.table-head, .table-row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr auto;
  gap: 8px;
  align-items: center;
}
.table-actions {
  margin-top: 8px;
}
</style>
