<template>
  <div class="text-left">
    <label class="block">
      <div class="flex flex-row">
        <span class="grow">{{ props.label }}</span>
        <input
          v-model="model"
          type="checkbox"
          class="peer appearance-none w-11 h-5 bg-transparent rounded-full checked:bg-primary cursor-pointer transition-colors duration-300"
        />
      </div>
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
