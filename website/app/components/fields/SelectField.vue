<template>
  <div class="text-left">
    <label class="block">
      <span>{{ props.label }}</span>
      <select
        v-model="model"
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      >
        <option v-for="item in props.items" :key="String(item)" :value="item">
          {{ item }}
        </option>
      </select>
    </label>
  </div>
</template>

<script setup lang="ts" generic="T">
const model = defineModel<T | undefined>();

const props = defineProps<{
  label: string;
  items: Array<T>;
}>();
</script>
