<template>
  <div class="text-left">
    <label class="block">
      <span>{{ props.label }}</span>
      <input
        id="field"
        v-model="model"
        :type="props.type"
        :placeholder="props.placeholder"
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </label>
  </div>
</template>

<script setup lang="ts">
import type { ValidationRule } from "@/composables/formValidation";

const model = defineModel<string>();

const props = withDefaults(
  defineProps<{
    label: string;
    placeholder: string;
    type?: HTMLInputElement["type"];
    validationRules: ValidationRule<string>[];
  }>(),
  {
    type: "text",
  },
);
</script>
