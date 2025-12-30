<template>
  <div class="body-editor">
    <div class="form">
      <label>
        Body 类型：
        <select v-model="localBodyType">
          <option value="none">none</option>
          <option value="raw">raw</option>
          <option value="x-www-form-urlencoded">x-www-form-urlencoded</option>
          <option value="form-data">form-data</option>
          <option value="binary">binary</option>
        </select>
      </label>
    </div>
    <div v-if="localBodyType==='raw'" class="form">
      <textarea v-model="rawText" rows="8" placeholder="文本内容"></textarea>
    </div>
    <div v-else-if="localBodyType==='x-www-form-urlencoded'" class="table">
      <div class="table-head">
        <span>Key</span><span>Value</span><span>操作</span>
      </div>
      <div class="table-row" v-for="(row,i) in urlEncodedRows" :key="'u-'+i">
        <input v-model="row.key" placeholder="key" />
        <input v-model="row.value" placeholder="value" />
        <button @click="removeUrlEncoded(i)">删除</button>
      </div>
      <div class="table-actions">
        <button @click="addUrlEncoded">新增一行</button>
      </div>
    </div>
    <div v-else-if="localBodyType==='form-data'" class="table">
      <div class="table-head form-data">
        <span>Key</span><span>Value</span><span>Description</span><span>操作</span>
      </div>
      <div class="table-row form-data" v-for="(row,i) in formDataRows" :key="'f-'+i">
        <input v-model="row.key" placeholder="key" />
        <input v-model="row.value" placeholder="value" />
        <input v-model="row.description" placeholder="description" />
        <button @click="removeFormData(i)">删除</button>
      </div>
      <div class="table-actions">
        <button @click="addFormData">新增一行</button>
      </div>
    </div>
    <div v-else-if="localBodyType==='binary'" class="form">
      <input type="file" @change="onBinaryFileChange" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

type BodyType = 'none' | 'raw' | 'x-www-form-urlencoded' | 'form-data' | 'binary';
type UrlEncodedRow = { key: string; value: string };
type FormDataRow = { key: string; value: string; description?: string };
type BinaryContent = { fileName?: string; file?: File };

const props = defineProps<{
  bodyType: BodyType;
  bodyContent: string | UrlEncodedRow[] | FormDataRow[] | BinaryContent;
}>();

const emit = defineEmits(['update:bodyType', 'update:bodyContent']);

const localBodyType = ref<BodyType>(props.bodyType);

const rawText = ref<string>(props.bodyType === 'raw' ? (typeof props.bodyContent === 'string' ? String(props.bodyContent) : '') : '');
const urlEncodedRows = ref<UrlEncodedRow[]>(props.bodyType === 'x-www-form-urlencoded' ? (Array.isArray(props.bodyContent) ? props.bodyContent as UrlEncodedRow[] : []) : []);
const formDataRows = ref<FormDataRow[]>(props.bodyType === 'form-data' ? (Array.isArray(props.bodyContent) ? props.bodyContent as FormDataRow[] : []) : []);
const binaryContent = ref<BinaryContent>(props.bodyType === 'binary' ? (typeof props.bodyContent === 'object' && !Array.isArray(props.bodyContent) && props.bodyContent !== null ? props.bodyContent as BinaryContent : {}) : {});

// 监听 props 变化，同步到本地状态
watch(() => props.bodyType, (newType) => {
  localBodyType.value = newType;
});

watch(() => [props.bodyType, props.bodyContent], ([newType, newContent]) => {
  if (newType === 'raw') {
    rawText.value = typeof newContent === 'string' ? String(newContent) : '';
  } else if (newType === 'x-www-form-urlencoded') {
    urlEncodedRows.value = Array.isArray(newContent) ? newContent as UrlEncodedRow[] : [];
  } else if (newType === 'form-data') {
    formDataRows.value = Array.isArray(newContent) ? newContent as FormDataRow[] : [];
  } else if (newType === 'binary') {
    binaryContent.value = typeof newContent === 'object' && !Array.isArray(newContent) && newContent !== null ? newContent as BinaryContent : {};
  }
}, { immediate: true });

watch(localBodyType, (t) => {
  emit('update:bodyType', t);
  if (t === 'raw') {
    if (!rawText.value) rawText.value = '';
    emit('update:bodyContent', rawText.value);
  } else if (t === 'x-www-form-urlencoded') {
    urlEncodedRows.value = urlEncodedRows.value.length ? urlEncodedRows.value : [{ key: '', value: '' }];
    emit('update:bodyContent', urlEncodedRows.value);
  } else if (t === 'form-data') {
    formDataRows.value = formDataRows.value.length ? formDataRows.value : [{ key: '', value: '', description: '' }];
    emit('update:bodyContent', formDataRows.value);
  } else if (t === 'binary') {
    emit('update:bodyContent', binaryContent.value);
  } else {
    emit('update:bodyContent', '');
  }
});

watch(rawText, (val) => {
  if (localBodyType.value === 'raw') {
    emit('update:bodyContent', val);
  }
});

watch(urlEncodedRows, (val) => {
  if (localBodyType.value === 'x-www-form-urlencoded') {
    emit('update:bodyContent', val);
  }
}, { deep: true });

watch(formDataRows, (val) => {
  if (localBodyType.value === 'form-data') {
    emit('update:bodyContent', val);
  }
}, { deep: true });

const addUrlEncoded = () => urlEncodedRows.value.push({ key: '', value: '' });
const removeUrlEncoded = (i: number) => urlEncodedRows.value.splice(i, 1);
const addFormData = () => formDataRows.value.push({ key: '', value: '', description: '' });
const removeFormData = (i: number) => formDataRows.value.splice(i, 1);
const onBinaryFileChange = (ev: Event) => {
  const input = ev.target as HTMLInputElement;
  const file = input.files?.[0];
  binaryContent.value = { fileName: file?.name, file };
  emit('update:bodyContent', binaryContent.value);
};
</script>

<style scoped>
.body-editor {
  width: 100%;
}
.form {
  display: grid;
  gap: 12px;
}
.form label {
  display: flex;
  gap: 8px;
  align-items: center;
  font-size: 14px;
  color: #606266;
}
.form label:first-child {
  min-width: 80px;
}
.form select,
.form input,
.form textarea {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
  transition: border-color 0.3s;
}
.form select,
.form input[type="text"],
.form input[type="file"] {
  height: 36px;
}
.form textarea {
  width: 100%;
  resize: vertical;
  min-height: 100px;
  font-family: inherit;
}
.form select:focus,
.form input:focus,
.form textarea:focus {
  outline: none;
  border-color: #409eff;
}
.table {
  display: grid;
  gap: 8px;
}
.table-head, .table-row {
  display: grid;
  gap: 8px;
  align-items: center;
}
.table-head {
  grid-template-columns: 1fr 1fr auto;
  font-weight: 500;
  color: #606266;
  font-size: 14px;
}
.table-row {
  grid-template-columns: 1fr 1fr auto;
}
.table-head.form-data, .table-row.form-data {
  grid-template-columns: 1fr 1fr 1fr auto;
}
.table-row input {
  padding: 8px 12px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  font-size: 14px;
  height: 36px;
  box-sizing: border-box;
  transition: border-color 0.3s;
}
.table-row input:focus {
  outline: none;
  border-color: #409eff;
}
.table-row button {
  padding: 6px 16px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #fff;
  color: #606266;
  font-size: 14px;
  height: 36px;
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.3s;
}
.table-row button:hover {
  color: #409eff;
  border-color: #c6e2ff;
  background-color: #ecf5ff;
}
.table-actions {
  margin-top: 8px;
}
.table-actions button {
  padding: 8px 20px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #fff;
  color: #606266;
  font-size: 14px;
  height: 36px;
  cursor: pointer;
  transition: all 0.3s;
}
.table-actions button:hover {
  color: #409eff;
  border-color: #409eff;
  background-color: #ecf5ff;
}
</style>
