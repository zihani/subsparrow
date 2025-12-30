<template>
  <div class="tree-container">
    <TreeNode
      v-for="item in data"
      :key="item.id"
      :node="item"
      :multiple="multiple"
      :selected-ids="modelValue"
      @select="onNodeClick"
    />
  </div>
</template>

<script setup>
import TreeNode from './treeNode.vue';

const props = defineProps({
  data: Array,
  multiple: Boolean,
  modelValue: [Array, String, Number] // 支持单选的 ID 或多选的 ID 数组
});

const emit = defineEmits(['update:modelValue', 'node-click']);

const onNodeClick = (node) => {
  if (props.multiple) {
    const res = [...props.modelValue];
    const index = res.indexOf(node.id);
    if (index > -1) {
      res.splice(index, 1); // 反选
    } else {
      res.push(node.id); // 选中
    }
    emit('update:modelValue', res);
  } else {
    // 单选逻辑
    emit('update:modelValue', node.id);
  }
  emit('node-click', node);
};
</script>