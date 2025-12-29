<template>
  <div class="api-debugger-card">
    <div class="request-line">
      <select v-model="method" class="method-select">
        <option v-for="item in methodsOptions" :key="item" :value="item">{{ item }}</option>
      </select>

      <textarea
        v-if="isMultiline"
        v-model="url"
        rows="3"
        placeholder="请输入请求 URL (http:// 或 https://)"
        class="url-input"
        :class="{ 'is-error': urlError }"
      />
      <input
        v-else
        v-model="url"
        type="text"
        placeholder="请输入请求 URL (http:// 或 https://)"
        class="url-input"
        :class="{ 'is-error': urlError }"
      />

      <label class="switch">
        <input type="checkbox" v-model="isMultiline" />
        <span>{{ isMultiline ? '多行' : '单行' }}</span>
      </label>

      <button class="primary" @click="sendRequest">发送</button>
    </div>
    <div class="save-line">
      <button class="warning" @click="saveToLocal">存储到本地</button>
      <button @click="clearData">重置</button>
    </div>
    <div v-if="urlError" class="url-error-tip">{{ urlError }}</div>
    <div v-if="uiMessage.text" :class="['ui-msg', uiMessage.type]">{{ uiMessage.text }}</div>
    
    <div class="tabs request-tabs">
      <div class="tab-nav">
        <button :class="{active: activeRequestTab==='params'}" @click="activeRequestTab='params'">Params</button>
        <button :class="{active: activeRequestTab==='authorization'}" @click="activeRequestTab='authorization'">Authorization</button>
        <button :class="{active: activeRequestTab==='headers'}" @click="activeRequestTab='headers'">Headers</button>
        <button :class="{active: activeRequestTab==='body'}" @click="activeRequestTab='body'">Body</button>
      </div>
      <div class="tab-content">
        <ParamsEditor
          v-if="activeRequestTab==='params'"
          :params="requestConfig.params"
          @update:params="requestConfig.params = $event"
        />
        <AuthorizationConfig
          v-else-if="activeRequestTab==='authorization'"
          :authorization="requestConfig.authorization"
          :auth-types="authTypes"
          @update:authorization="requestConfig.authorization = $event"
        />
        <HeadersEditor
          v-else-if="activeRequestTab==='headers'"
          :headers="requestConfig.headers"
          @update:headers="requestConfig.headers = $event"
        />
        <BodyEditor
          v-else
          :body-type="requestConfig.body.type"
          :body-content="requestConfig.body.content"
          @update:body-type="requestConfig.body.type = $event"
          @update:body-content="requestConfig.body.content = $event"
        />
      </div>
    </div>

    <hr />

    <h3>返回数据格式</h3>
    <div class="tabs response-tabs">
      <div class="tab-nav">
        <button :class="{active: activeResponseTab==='responseBody'}" @click="activeResponseTab='responseBody'">Body</button>
        <button :class="{active: activeResponseTab==='responseHeaders'}" @click="activeResponseTab='responseHeaders'">Headers</button>
        <button :class="{active: activeResponseTab==='responseInfo'}" @click="activeResponseTab='responseInfo'">Info</button>
      </div>
      <div class="tab-content">
        <ResponseDisplay
          v-if="activeResponseTab==='responseBody'"
          :response-data="responseData"
        />
        <ResponseHeadersDisplay
          v-else-if="activeResponseTab==='responseHeaders'"
          :response-headers="responseHeaders"
        />
        <ResponseInfoDisplay
          v-else
          :response-status="responseStatus"
          :response-time="responseTime"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue';
import axios from 'axios';
import ParamsEditor from './apiPost/ParamsEditor.vue';
import AuthorizationConfig from './apiPost/AuthorizationConfig.vue';
import HeadersEditor from './apiPost/HeadersEditor.vue';
import BodyEditor from './apiPost/BodyEditor.vue';
import ResponseDisplay from './apiPost/ResponseDisplay.vue';
import ResponseHeadersDisplay from './apiPost/ResponseHeadersDisplay.vue';
import ResponseInfoDisplay from './apiPost/ResponseInfoDisplay.vue';
// 可选：根据项目需要集成路由或状态管理
// --- 类型定义 ---

// 定义支持的 HTTP 方法
type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'HEAD' | 'OPTIONS' | 'TRACE' | 'CONNECT';
type AuthType =
  | 'Basic Auth' | 'Bearer Token' | 'JWT Bearer' | 'Digest Auth'
  | 'OAuth 1.0' | 'OAuth 2.0' | 'Hawk Authentication' | 'AWS Signature'
  | 'NTLM Authentication' | 'API Key' | 'Akamai EdgeGrid' | 'ASAP (AtIassian)' | 'None';
type ParamRow = { key: string; value: string; description?: string };
type HeaderRow = { key: string; value: string; description?: string };
type BodyType = 'none' | 'raw' | 'x-www-form-urlencoded' | 'form-data' | 'binary';
type UrlEncodedRow = { key: string; value: string };
type FormDataRow = { key: string; value: string; description?: string };
type BinaryContent = { fileName?: string; file?: File };
type RequestConfig = {
  params: ParamRow[];
  authorization: { type: AuthType; token: string };
  headers: HeaderRow[];
  body: { type: BodyType; content: string | UrlEncodedRow[] | FormDataRow[] | BinaryContent };
};


// --- 核心逻辑 ---

// 1. URL 和方法
const methodsOptions: HttpMethod[] = ['GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS', 'TRACE', 'CONNECT'];
const method = ref<HttpMethod>('GET');
const url = ref<string>('');
const isMultiline = ref<boolean>(false);
const uiMessage = ref<{ text: string; type: 'success'|'error'|'' }>({ text: '', type: '' });

// HTTP/HTTPS 正则表达式
// 匹配 http:// 或 https:// 开头，后面跟着至少一个字符 (域名/IP)
const urlRegex = /^(https?:\/\/[^\s$.?#].[^\s]*)$/i;

const urlError = computed(() => {
  const value = url.value;
  if (!isMultiline.value) {
    if (value.length === 0) return 'URL 不能为空';
    return urlRegex.test(value) ? '' : 'URL 格式不正确，请确保以 http:// 或 https:// 开头';
  }
  const lines = value.split('\n').map(l => l.trim()).filter(l => l.length > 0);
  if (lines.length === 0) return '至少填写一行 URL';
  const invalid = lines.find(l => !urlRegex.test(l));
  return invalid ? '存在格式错误的 URL 行' : '';
});

// 2. 请求配置
const activeRequestTab = ref('params');
const requestConfig = ref<RequestConfig>({
  params: [{ key: '', value: '', description: '' }],
  authorization: { type: 'None', token: '' },
  headers: [{ key: '', value: '', description: '' }],
  body: { type: 'raw', content: '{\n  \n}' },
});
const authTypes: AuthType[] = [
  'Basic Auth','Bearer Token','JWT Bearer','Digest Auth','OAuth 1.0','OAuth 2.0',
  'Hawk Authentication','AWS Signature','NTLM Authentication','API Key','Akamai EdgeGrid','ASAP (AtIassian)','None',
];
onMounted(() => {
  try {
    const raw = localStorage.getItem('apiPost:last')
    const payload = raw ? JSON.parse(raw) : null
    if (payload) {
      method.value = payload.method
      url.value = payload.url
      requestConfig.value = payload.config as RequestConfig
    }
  } catch {}
})

// 3. 响应结果
const activeResponseTab = ref('responseBody');
const responseData = ref<any>(null);
const responseHeaders = ref<Record<string, string>>({});
const responseStatus = ref<number | null>(null);
const responseTime = ref<number | null>(null);

/**
 * 模拟发送请求
 */
const sendRequest = async () => {
  if (urlError.value) {
    uiMessage.value = { text: urlError.value, type: 'error' };
    return;
  }
  const targetUrl = isMultiline.value
    ? (url.value.split('\n').map(l => l.trim()).filter(Boolean)[0] || '')
    : url.value;
  if (!targetUrl) {
    uiMessage.value = { text: 'URL 不能为空', type: 'error' };
    return;
  }

  // 构建 headers
  const headers: Record<string, string> = {};
  for (const h of requestConfig.value.headers) {
    if (h.key && h.value) headers[h.key] = h.value;
  }
  // Authorization 映射
  const auth = requestConfig.value.authorization;
  if (auth?.type === 'Bearer Token' || auth?.type === 'JWT Bearer') {
    headers['Authorization'] = `Bearer ${auth.token}`;
  } else if (auth?.type === 'Basic Auth') {
    headers['Authorization'] = `Basic ${auth.token}`;
  } else if (auth?.type === 'API Key' && !headers['x-api-key']) {
    headers['x-api-key'] = auth.token;
  }

  // 构建 params
  const params: Record<string, string> = {};
  for (const p of requestConfig.value.params) {
    if (p.key && p.value !== undefined) params[p.key] = p.value;
  }

  // 构建 body
  let data: any = undefined;
  const bodyType = requestConfig.value.body.type;
  if (bodyType === 'raw') {
    data = requestConfig.value.body.content as string;
    if (!headers['Content-Type']) headers['Content-Type'] = 'text/plain';
  } else if (bodyType === 'x-www-form-urlencoded') {
    const rows = requestConfig.value.body.content as { key: string; value: string }[];
    const formStr = (rows || [])
      .filter(r => r.key)
      .map(r => `${encodeURIComponent(r.key)}=${encodeURIComponent(r.value ?? '')}`)
      .join('&');
    data = formStr;
    headers['Content-Type'] = 'application/x-www-form-urlencoded';
  } else if (bodyType === 'form-data') {
    const rows = requestConfig.value.body.content as { key: string; value: string; description?: string; type?: string; fileName?: string }[];
    const fd = new FormData();
    (rows || []).forEach(r => {
      if (!r.key) return;
      fd.append(r.key, r.value ?? '');
    });
    data = fd;
    // Content-Type 由浏览器自动设置为 multipart/form-data，含边界
    delete headers['Content-Type'];
  } else if (bodyType === 'binary') {
    const bc = requestConfig.value.body.content as { fileName?: string; file?: File };
    if (bc?.file) {
      data = bc.file;
      if (!headers['Content-Type']) headers['Content-Type'] = 'application/octet-stream';
    } else {
      data = '';
    }
  } else {
    data = undefined;
  }

  // 创建 axios 实例
  const client = axios.create({
    // 不设置 baseURL，允许完整 URL 直发；如需统一 base，可改用 import.meta.env.VITE_API_BASE
    timeout: 10000,
    headers,
  });

  const started = performance.now();
  try {
    const res = await client.request({
      method: method.value,
      url: targetUrl,
      params,
      data,
    });
    const elapsed = Math.round(performance.now() - started);
    responseData.value = res.data;
    responseHeaders.value = Object.fromEntries(Object.entries(res.headers || {}).map(([k, v]) => [k, String(v)]));
    responseStatus.value = res.status;
    responseTime.value = elapsed;
    uiMessage.value = { text: '请求成功', type: 'success' };
    activeResponseTab.value = 'responseBody';
  } catch (e: any) {
    const elapsed = Math.round(performance.now() - started);
    responseData.value = e?.response?.data ?? { message: e?.message ?? 'Request failed' };
    responseHeaders.value = Object.fromEntries(Object.entries(e?.response?.headers || {}).map(([k, v]) => [k, String(v)]));
    responseStatus.value = e?.response?.status ?? null;
    responseTime.value = elapsed;
    uiMessage.value = { text: '请求失败', type: 'error' };
    activeResponseTab.value = 'responseBody';
  }
};

const saveToLocal = () => {
  if (urlError.value) {
    uiMessage.value = { text: urlError.value, type: 'error' };
    return;
  }
  const payload = {
    method: method.value,
    url: url.value,
    config: JSON.parse(JSON.stringify(requestConfig.value)) as RequestConfig,
    savedAt: new Date().toISOString(),
  }
  try {
    localStorage.setItem('apiPost:last', JSON.stringify(payload))
    uiMessage.value = { text: '已存储到本地', type: 'success' };
  } catch {
    uiMessage.value = { text: '存储失败', type: 'error' };
  }
}

const clearData = () => {
  method.value = 'GET'
  url.value = ''
  requestConfig.value = {
    params: [{ key: '', value: '', description: '' }],
    authorization: { type: 'None', token: '' },
    headers: [{ key: '', value: '', description: '' }],
    body: { type: 'raw', content: '' },
  }
  try {
    localStorage.removeItem('apiPost:last')
  } catch {}
  uiMessage.value = { text: '已清空输入数据', type: 'success' };
}
// 监听 URL 变化，方便调试时查看验证效果
// watch(url, (newVal) => {
//   console.log('URL 验证结果:', urlError.value ? '错误' : '通过');
// });
</script>

<style scoped>
.api-debugger-card {
  max-width: 100%;
  margin: 20px auto;
}

.request-line {
  display: flex;
  gap: 10px;
  align-items: center;
}

.method-select {
  width: 120px;
  flex-shrink: 0;
}

.url-input {
  flex-grow: 1;
  padding: 8px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
}
.is-error {
  border-color: #f56c6c;
}
.url-error-tip {
  color: #f56c6c;
  font-size: 12px;
  margin-top: 5px;
  margin-bottom: 10px;
}
.switch {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}
.primary {
  background: #409eff;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
}
.warning {
  background: #e6a23c;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
}
.ui-msg {
  margin-top: 8px;
  font-size: 12px;
}
.ui-msg.success { color: #67c23a; }
.ui-msg.error { color: #f56c6c; }
.request-tabs,
.response-tabs {
  margin-top: 15px;
}
.tabs .tab-nav {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
}
.tabs .tab-nav button {
  padding: 6px 12px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #fff;
}
.tabs .tab-nav button.active {
  background: #409eff;
  color: #fff;
  border-color: #409eff;
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
.response-pre {
  padding: 8px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #f9fafc;
  white-space: pre-wrap;
}
.info {
  display: grid;
  gap: 6px;
}
</style>
