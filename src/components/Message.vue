<template>
  <Teleport to="body">
    <transition name="message-fade">
      <div
        v-if="visible"
        class="message-wrapper"
      >
        <div class="message" :class="`message--${type}`">
          <span class="message__icon">
            {{ icon }}
          </span>
          <span class="message__content">
            {{ message }}
          </span>
          <button
            v-if="closable"
            class="message__close"
            type="button"
            @click="close"
          >
            ×
          </button>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, onUnmounted, watch } from 'vue';

type MessageType = 'success' | 'warning' | 'info' | 'error';

const props = withDefaults(defineProps<{
  modelValue: boolean;
  type?: MessageType;
  message: string;
  duration?: number; // 显示时长（毫秒），0 或负数时不自动关闭
  closable?: boolean;
}>(), {
  type: 'info',
  duration: 3000,
  closable: true,
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
  (e: 'close'): void;
}>();

const visible = computed({
  get: () => props.modelValue,
  set: (val: boolean) => emit('update:modelValue', val),
});

let timer: ReturnType<typeof setTimeout> | null = null;

const clearTimer = () => {
  if (timer) {
    clearTimeout(timer);
    timer = null;
  }
};

const startTimer = () => {
  clearTimer();
  if (props.duration && props.duration > 0) {
    timer = setTimeout(() => {
      close();
    }, props.duration);
  }
};

const close = () => {
  clearTimer();
  visible.value = false;
  emit('close');
};

watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      startTimer();
    } else {
      clearTimer();
    }
  },
  { immediate: true },
);

onUnmounted(() => {
  clearTimer();
});

const icon = computed(() => {
  switch (props.type) {
    case 'success':
      return '✔';
    case 'warning':
      return '⚠';
    case 'error':
      return '✖';
    default:
      return 'ℹ';
  }
});
</script>

<style scoped>
.message-wrapper {
  position: fixed;
  top: 80px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 3000;
  pointer-events: none;
}

.message {
  min-width: 260px;
  max-width: 480px;
  padding: 10px 16px;
  border-radius: 6px;
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.16);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  line-height: 1.4;
  color: #fff;
  pointer-events: auto;
}

.message__icon {
  font-size: 16px;
}

.message__content {
  flex: 1;
}

.message__close {
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  font-size: 16px;
  padding: 0 4px;
}

.message--success {
  background: #67c23a;
}

.message--warning {
  background: #e6a23c;
}

.message--info {
  background: #909399;
}

.message--error {
  background: #f56c6c;
}

.message-fade-enter-active,
.message-fade-leave-active {
  transition: all 0.25s ease-out;
}

.message-fade-enter-from,
.message-fade-leave-to {
  opacity: 0;
  transform: translate(-50%, -10px);
}
</style>


