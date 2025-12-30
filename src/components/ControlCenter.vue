<template>
  <div 
    ref="panelRef"
    class="control-center" 
    :class="{ 'is-open': modelValue }"
    :style="panelStyle"
  >
    <div class="panel-content">
      <slot>
        <div v-if="networkInterfaces.length > 0" class="network-section">
          <div class="section-header">
            <h3 class="section-title">网络接口信息</h3>
            <button @click="fetchNetworkInterfaces" class="refresh-btn">刷新</button>
          </div>
          <div class="network-list">
            <div 
              v-for="iface in networkInterfaces" 
              :key="iface.name"
              class="network-item"
            >
              <div class="network-details">
                <div class="network-detail">
                  <span class="detail-label">网络接口:</span>
                  <code class="detail-value">{{ iface.name }}</code>
                </div>
                <div class="network-detail">
                  <span class="detail-label">IP 地址:</span>
                  <code class="detail-value">{{ iface.ip }}</code>
                </div>
                <div class="network-detail">
                  <span class="detail-label">MAC 地址:</span>
                  <code class="detail-value">{{ iface.mac }}</code>
                </div>
              </div>
            </div>
          </div>
        </div>
      </slot>
    </div>
    <div 
      class="drag-handle" 
      @mousedown="handleDragStart"
      @touchstart="handleDragStart"
    >
      <div class="bar"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  modelValue: boolean;
  maxHeight?: number;
}

interface NetworkInterface {
  name: string;
  ip: string;
  mac: string;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: false,
  maxHeight: 400
});

const emit = defineEmits(['update:modelValue']);

// 响应式数据
const panelRef = ref<HTMLElement | null>(null);
const dragY = ref(0); // 当前实时偏移量
const isDragging = ref(false);
let startY = 0;

// 网络接口信息
const networkInterfaces = ref<NetworkInterface[]>([]);

// 获取网络接口信息
const fetchNetworkInterfaces = async () => {
  try {
    const interfaces = await invoke<NetworkInterface[]>('get_network_interfaces');
    networkInterfaces.value = interfaces
  } catch (error) {
    console.error('获取网络接口信息失败:', error);
  }
};

onMounted(() => {
  fetchNetworkInterfaces();
});

// 计算面板样式
const panelStyle = computed(() => {
  const hiddenOffset = -(props.maxHeight - 30); // 留出30px手柄
  let y = props.modelValue ? 0 : hiddenOffset;

  // 如果正在拖拽，根据拖拽距离实时计算
  if (isDragging.value) {
    y = Math.min(0, Math.max(hiddenOffset, dragY.value));
  }

  return {
    height: `${props.maxHeight}px`,
    transform: `translateY(${y}px)`,
    transition: isDragging.value ? 'none' : 'transform 0.5s cubic-bezier(0.16, 1, 0.3, 1)'
  };
});

// 手势处理
const handleDragStart = (e: MouseEvent | TouchEvent) => {
  isDragging.value = true;
  startY = 'touches' in e ? e.touches[0].clientY : e.clientY;
  
  // 获取当前面板的实时位置作为起始偏移
  const rect = panelRef.value?.getBoundingClientRect();
  dragY.value = rect ? rect.top : 0;

  window.addEventListener('mousemove', handleDragMove);
  window.addEventListener('touchmove', handleDragMove);
  window.addEventListener('mouseup', handleDragEnd);
  window.addEventListener('touchend', handleDragEnd);
};

const handleDragMove = (e: MouseEvent | TouchEvent) => {
  if (!isDragging.value) return;
  const currentY = 'touches' in e ? e.touches[0].clientY : e.clientY;
  const deltaY = currentY - startY;
  
  // 更新偏移量
  const hiddenOffset = -(props.maxHeight - 30);
  const basePos = props.modelValue ? 0 : hiddenOffset;
  dragY.value = basePos + deltaY;
};

const handleDragEnd = () => {
  isDragging.value = false;
  const hiddenOffset = -(props.maxHeight - 30);
  
  // 阈值判断：如果拉动超过1/4则切换状态
  const threshold = props.maxHeight / 4;
  if (props.modelValue && dragY.value < -threshold) {
    emit('update:modelValue', false);
  } else if (!props.modelValue && dragY.value > hiddenOffset + threshold) {
    emit('update:modelValue', true);
  }

  window.removeEventListener('mousemove', handleDragMove);
  window.removeEventListener('touchmove', handleDragMove);
  window.removeEventListener('mouseup', handleDragEnd);
  window.removeEventListener('touchend', handleDragEnd);
};
</script>

<style scoped>
.control-center {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  background: rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
  z-index: 2000;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
}

.panel-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 40px 24px;
  height: calc(100% - 30px);
  box-sizing: border-box;
  overflow-y: auto;
}

.default-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 16px;
}

.control-card {
  background: white;
  border-radius: 16px;
  padding: 16px;
  display: flex;
  align-items: center;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
  border: 1px solid rgba(0,0,0,0.05);
}

.control-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.05);
}

.control-card.active {
  background: #007aff;
  color: white;
}

.icon {
  font-size: 24px;
  margin-right: 16px;
}

.label {
  font-weight: 600;
  font-size: 15px;
}

.status {
  font-size: 12px;
  opacity: 0.7;
}

.drag-handle {
  position: absolute;
  bottom: 0;
  width: 100%;
  height: 30px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: grab;
}

.drag-handle:active {
  cursor: grabbing;
}

.bar {
  width: 40px;
  height: 5px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 10px;
}

.network-section {
  margin-top: 32px;
  padding-top: 24px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #333;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.refresh-btn {
  padding: 6px 12px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #fff;
  color: #606266;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s;
}

.refresh-btn:hover {
  color: #409eff;
  border-color: #409eff;
  background-color: #ecf5ff;
}

.network-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 8px;
}

.network-item {
  background: white;
  border-radius: 8px;
  padding: 12px;
  border: 1px solid rgba(0, 0, 0, 0.05);
  transition: all 0.2s ease;
}

.network-item:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.network-name {
  font-weight: 600;
  font-size: 13px;
  color: #333;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.network-details {
  display: grid;
  gap: 6px;
}

.network-detail {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
}

.detail-label {
  color: #666;
  min-width: 60px;
}

.detail-value {
  background: #f5f5f5;
  padding: 2px 6px;
  border-radius: 3px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'source-code-pro', monospace;
  font-size: 11px;
  color: #333;
  border: 1px solid #e0e0e0;
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
