<template>
  <div class="tree-node">
    <div 
      class="node-content" 
      :class="{ 'is-active': isSelected }"
      @click="handleSelect"
    >
      <span v-if="node.children" @click.stop="toggleExpand">
        {{ expanded ? '▼' : '▶' }}
      </span>
      
      <input 
        v-if="multiple" 
        type="checkbox" 
        :checked="isSelected" 
        @change="handleSelect"
      />
      
      <span>{{ node.label }}</span>
    </div>

    <div v-if="expanded && node.children" class="node-children">
      <TreeNode
        v-for="child in node.children"
        :key="child.id"
        :node="child"
        :multiple="multiple"
        :selected-ids="selectedIds"
        @select="$emit('select', $event)"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import TreeNode from './treeNode.vue';

const props = defineProps({
  node: Object,
  multiple: Boolean,
  selectedIds: [Array, String, Number]
});

const emit = defineEmits(['select']);
const expanded = ref(false);

const isSelected = computed(() => {
  if (props.multiple) {
    return props.selectedIds.includes(props.node.id);
  }
  return props.selectedIds === props.node.id;
});

const toggleExpand = () => expanded.value = !expanded.value;

const handleSelect = () => {
  emit('select', props.node);
};
</script>