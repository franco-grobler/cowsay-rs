<template>
  <div class="text-left">
    <label class="block">
      <span>{{ props.label }}</span>
      <textarea
        v-model="model"
        :rows="props.rows"
        :readonly="props.readonly"
        :class="props.textareaClass"
        class="mt-1 block h-full w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
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
    readonly?: boolean;
    rows?: number | string | undefined;
    textareaClass?: string;
    validationRules?: ValidationRule<string>[];
  }>(),
  {
    readonly: false,
    rows: undefined,
    textareaClass: undefined,
    validationRules: () => [],
  },
);
</script>
