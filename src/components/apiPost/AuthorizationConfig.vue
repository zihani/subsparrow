<template>
  <div class="auth-config">
    <div class="form">
      <label>
        类型：
        <select v-model="localAuth.type">
          <option v-for="opt in authTypes" :key="opt" :value="opt">{{ opt }}</option>
        </select>
      </label>
      <label>
        凭证：
        <input v-model="localAuth.token" placeholder="输入凭证或令牌" />
      </label>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

type AuthType =
  | 'Basic Auth' | 'Bearer Token' | 'JWT Bearer' | 'Digest Auth'
  | 'OAuth 1.0' | 'OAuth 2.0' | 'Hawk Authentication' | 'AWS Signature'
  | 'NTLM Authentication' | 'API Key' | 'Akamai EdgeGrid' | 'ASAP (AtIassian)' | 'None';

const props = defineProps<{
  authorization: { type: AuthType; token: string };
  authTypes: AuthType[];
}>();

const emit = defineEmits(['update:authorization']);

const localAuth = ref<{ type: AuthType; token: string }>({ ...props.authorization });

watch(localAuth, (newVal) => {
  emit('update:authorization', newVal);
}, { deep: true });
</script>

<style scoped>
.auth-config {
  padding: 8px 0;
  display: grid;
  gap: 8px;
}
.row {
  display: flex;
  gap: 8px;
  align-items: center;
}
</style>
