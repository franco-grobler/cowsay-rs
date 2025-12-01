<template>
  <div class="text-left">
    <label class="block">
      <span>{{ props.label }}</span>
      <input
        v-model="model"
        type="checkbox"
        class="peer appearance-none w-11 h-5 bg-slate-100 rounded-full checked:bg-slate-800 cursor-pointer transition-colors duration-300"
      />
    </label>
  </div>
</template>

<script setup lang="ts">
const model = defineModel<boolean>({ default: false, required: true });

const props = withDefaults(
  defineProps<{
    label: string;
  }>(),
  {},
);
</script>
