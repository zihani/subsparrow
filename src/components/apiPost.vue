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
      <button class="warning" @click="importFromFile">导入配置</button>
      <button class="export-btn" @click="exportToFile">导出</button>
      <button @click="clearData">重置</button>
    </div>
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

    <!-- Tree 测试展示 -->
    <div class="tree-demo">
      <h3>Tree 组件测试数据</h3>
      <Tree :data="treeTestData" />
    </div>

    <!-- Message 组件 -->
    <Message
      v-model="showMessage"
      :type="messageType"
      :message="messageText"
      :duration="3000"
      :closable="true"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue';
import axios from 'axios';
import { open, save } from '@tauri-apps/plugin-dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
import ParamsEditor from './apiPost/ParamsEditor.vue';
import AuthorizationConfig from './apiPost/AuthorizationConfig.vue';
import HeadersEditor from './apiPost/HeadersEditor.vue';
import BodyEditor from './apiPost/BodyEditor.vue';
import ResponseDisplay from './apiPost/ResponseDisplay.vue';
import ResponseHeadersDisplay from './apiPost/ResponseHeadersDisplay.vue';
import ResponseInfoDisplay from './apiPost/ResponseInfoDisplay.vue';
import Message from './Message.vue';
import Tree from './TreeNode/Tree.vue';
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

// Message 组件状态
const showMessage = ref(false);
const messageType = ref<'success' | 'warning' | 'info' | 'error'>('info');
const messageText = ref('');

// 显示消息的辅助函数
const showMsg = (type: 'success' | 'warning' | 'info' | 'error', text: string, duration = 3000) => {
  messageType.value = type;
  messageText.value = text;
  showMessage.value = true;
};

// Tree 测试数据
const treeTestData = ref([
  {
    id: 1,
    label: '根节点 1',
    children: [
      {
        id: 11,
        label: '子节点 1-1',
        children: [
          { id: 111, label: '子节点 1-1-1' },
          { id: 112, label: '子节点 1-1-2' },
        ],
      },
      {
        id: 12,
        label: '子节点 1-2',
      },
    ],
  },
  {
    id: 2,
    label: '根节点 2',
    children: [
      { id: 21, label: '子节点 2-1' },
      { id: 22, label: '子节点 2-2' },
    ],
  },
]);


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
  // URL 验证
  if (urlError.value) {
    showMsg('error', urlError.value);
    return;
  }
  
  const targetUrl = isMultiline.value
    ? (url.value.split('\n').map(l => l.trim()).filter(Boolean)[0] || '')
    : url.value;
  if (!targetUrl) {
    showMsg('error', 'URL 不能为空');
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
      url: url.value,
      params,
      data,
    });
    const elapsed = Math.round(performance.now() - started);
    responseData.value = res.data;
    responseHeaders.value = Object.fromEntries(Object.entries(res.headers || {}).map(([k, v]) => [k, String(v)]));
    responseStatus.value = res.status;
    responseTime.value = elapsed;
    showMsg('success', `请求成功 (${res.status})`, 2000);
    activeResponseTab.value = 'responseBody';
  } catch (e: any) {
    const elapsed = Math.round(performance.now() - started);
    responseData.value = e?.response?.data ?? { message: e?.message ?? 'Request failed' };
    responseHeaders.value = Object.fromEntries(Object.entries(e?.response?.headers || {}).map(([k, v]) => [k, String(v)]));
    responseStatus.value = e?.response?.status ?? null;
    responseTime.value = elapsed;
    const errorMsg = e?.response?.data?.message || e?.message || '请求失败';
    showMsg('error', errorMsg);
    activeResponseTab.value = 'responseBody';
  }
};

const saveToLocal = () => {
  // 仍然保留本地存储逻辑，方便自动恢复
  const payload = {
    method: method.value,
    url: url.value,
    config: JSON.parse(JSON.stringify(requestConfig.value)) as RequestConfig,
    savedAt: new Date().toISOString(),
  }
  try {
    localStorage.setItem('apiPost:last', JSON.stringify(payload));
  } catch (error) {
    console.error('本地存储失败:', error);
  }
}

// 从 JSON 文件导入配置
const importFromFile = async () => {
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'JSON',
          extensions: ['json'],
        },
      ],
    });

    if (!selected) {
      // 用户取消
      return;
    }

    const filePath = Array.isArray(selected) ? selected[0] : selected;
    if (!filePath) return;

    const content = await readTextFile(filePath);

    let data: any;
    try {
      data = JSON.parse(content);
    } catch {
      showMsg('error', '配置文件不是有效的 JSON');
      return;
    }

    if (!data || typeof data !== 'object') {
      showMsg('error', '配置文件内容格式不正确');
      return;
    }

    // method
    if (data.method && (methodsOptions as string[]).includes(data.method)) {
      method.value = data.method as HttpMethod;
    }

    // URL & protocol
    let importedUrl: string = (data.url || '').trim();
    const protocol: string | undefined = data.protocol;
    if (importedUrl) {
      const clean = importedUrl.replace(/^https?:\/\//i, '');
      if (protocol === 'https') {
        importedUrl = `https://${clean}`;
      } else if (protocol === 'http') {
        importedUrl = `http://${clean}`;
      } else if (!/^https?:\/\//i.test(importedUrl)) {
        // 默认 http
        importedUrl = `http://${clean}`;
      }
      url.value = importedUrl;
    }

    // Params
    if (Array.isArray(data.params)) {
      requestConfig.value.params = data.params.map((p: any) => ({
        key: String(p.key ?? ''),
        value: String(p.value ?? ''),
        description: p.description ? String(p.description) : '',
      })) as ParamRow[];
    }

    // Authorization
    if (data.authorization && typeof data.authorization === 'object') {
      const a = data.authorization;
      if (a.type && (authTypes as string[]).includes(a.type)) {
        requestConfig.value.authorization.type = a.type as AuthType;
      }
      if (typeof a.token === 'string') {
        requestConfig.value.authorization.token = a.token;
      }
    }

    // Headers
    if (Array.isArray(data.headers)) {
      requestConfig.value.headers = data.headers.map((h: any) => ({
        key: String(h.key ?? ''),
        value: String(h.value ?? ''),
        description: h.description ? String(h.description) : '',
      })) as HeaderRow[];
    }

    // Body
    if (data.body && typeof data.body === 'object') {
      const b = data.body;
      if (b.type && ['none', 'raw', 'x-www-form-urlencoded', 'form-data', 'binary'].includes(b.type)) {
        requestConfig.value.body.type = b.type as BodyType;
      }
      if ('content' in b) {
        requestConfig.value.body.content = b.content as any;
      }
    }

    // 导入后存一份到 localStorage
    saveToLocal();
    showMsg('success', '配置导入成功', 2000);
  } catch (error: any) {
    console.error('导入配置失败:', error);
    showMsg('error', error?.message || '导入配置失败');
  }
}

// 导出请求配置为 JSON 文件
const exportToFile = async () => {
  // URL 验证
  if (urlError.value) {
    showMsg('error', urlError.value);
    return;
  }

  if (!url.value.trim()) {
    showMsg('error', 'URL 不能为空');
    return;
  }

  try {
    // 解析 URL 获取协议
    let protocol = 'http';
    try {
      const urlObj = new URL(url.value.trim());
      protocol = urlObj.protocol.replace(':', '');
    } catch {
      // 如果 URL 解析失败，尝试从字符串中提取
      if (url.value.trim().startsWith('https://')) {
        protocol = 'https';
      }
    }

    // 构建导出的 JSON 数据（符合规范格式）
    const exportData = {
      // 请求方式
      method: method.value,
      // 协议类型 (http/https)
      protocol: protocol,
      // 完整 URL
      url: url.value.trim(),
      // Params 参数
      params: requestConfig.value.params
        .filter(p => p.key && p.value)
        .map(p => ({
          key: p.key,
          value: p.value,
          description: p.description || ''
        })),
      // Authorization 参数
      authorization: {
        type: requestConfig.value.authorization.type,
        token: requestConfig.value.authorization.token
      },
      // Headers 参数
      headers: requestConfig.value.headers
        .filter(h => h.key && h.value)
        .map(h => ({
          key: h.key,
          value: h.value,
          description: h.description || ''
        })),
      // Body 参数
      body: {
        type: requestConfig.value.body.type,
        content: requestConfig.value.body.content
      },
      // 元数据
      metadata: {
        exportedAt: new Date().toISOString(),
        version: '1.0.0'
      }
    };

    // 打开保存文件对话框
    const filePath = await save({
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }],
      defaultPath: `api-request-${Date.now()}.json`
    });

    if (filePath) {
      // 将数据格式化为 JSON 字符串（美化格式）
      const jsonContent = JSON.stringify(exportData, null, 2);
      await writeTextFile(filePath, jsonContent);
      showMsg('success', '导出成功', 2000);
    }
  } catch (error: any) {
    console.error('导出失败:', error);
    showMsg('error', error?.message || '导出失败');
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
    showMsg('success', '已清空输入数据', 2000);
  } catch {
    showMsg('error', '清空数据失败');
  }
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
  align-items: stretch;
}

.method-select {
  width: 100px;
  flex-shrink: 0;
  padding: 8px 12px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #fff;
  font-size: 14px;
  height: 36px;
  box-sizing: border-box;
}

.url-input {
  flex-grow: 1;
  padding: 8px 12px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  font-size: 14px;
  height: 36px;
  box-sizing: border-box;
}

.url-input[type="text"] {
  height: 36px;
}

.url-input[rows] {
  height: auto;
  min-height: 36px;
  resize: vertical;
}

.is-error {
  border-color: #f56c6c;
}

.switch {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  white-space: nowrap;
  flex-shrink: 0;
}

.switch input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.primary {
  background: #409eff;
  color: white;
  border: none;
  padding: 8px 20px;
  border-radius: 4px;
  font-size: 14px;
  height: 36px;
  cursor: pointer;
  white-space: nowrap;
  flex-shrink: 0;
  transition: background-color 0.3s;
}

.primary:hover {
  background: #66b1ff;
}

.primary:active {
  background: #3a8ee6;
}

.warning {
  background: #e6a23c;
  color: white;
  border: none;
  padding: 8px 20px;
  border-radius: 4px;
  font-size: 14px;
  height: 36px;
  cursor: pointer;
  white-space: nowrap;
  transition: background-color 0.3s;
}

.warning:hover {
  background: #ebb563;
}

.warning:active {
  background: #cf9236;
}

.save-line {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.save-line button {
  padding: 8px 20px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  font-size: 14px;
  height: 36px;
  cursor: pointer;
  background: #fff;
  transition: all 0.3s;
}

.save-line button:hover {
  background: #f5f7fa;
  border-color: #c0c4cc;
}

.export-btn {
  background: #409eff;
  color: white;
  border: none;
  padding: 8px 20px;
  border-radius: 4px;
  font-size: 14px;
  height: 36px;
  cursor: pointer;
  transition: all 0.3s;
}

.export-btn:hover {
  background: #66b1ff;
  border-color: #409eff;
}

.export-btn:active {
  background: #3a8ee6;
}
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

::-webkit-scrollbar {
  width: 1px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 1px;
}
</style>
