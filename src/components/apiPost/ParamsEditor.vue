<template>
  <div class="params-editor">
    <div class="table">
      <div class="table-head">
        <span>Key</span><span>Value</span><span>Description</span><span>操作</span>
      </div>
      <div class="table-row" v-for="(row, i) in localParams" :key="'p-'+i">
        <input v-model="row.key" placeholder="key" />
        <input v-model="row.value" placeholder="value" />
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

interface ParamRow {
  key: string;
  value: string;
  description?: string;
}

const props = defineProps<{
  params: ParamRow[];
}>();

const emit = defineEmits(['update:params']);

const localParams = ref<ParamRow[]>(props.params);

watch(localParams, (newVal) => {
  emit('update:params', newVal);
}, { deep: true });

const addRow = () => {
  localParams.value.push({ key: '', value: '', description: '' });
};

const removeRow = (index: number) => {
  localParams.value.splice(index, 1);
};
</script>

<style scoped>
.params-editor {
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
