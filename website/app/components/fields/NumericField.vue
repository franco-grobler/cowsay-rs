<template>
  <div class="text-left">
    <label class="block">
      <div class="flex flex-row">
        <span class="grow">{{ props.label }}</span>
        <input
          v-model="model"
          type="numeric"
          :placeholder="props.placeholder"
          class="rounded-md border-gray-300 shadow-sm grow-0 max-w-[50%] text-right px-2"
          :class="{
            'border-red-600 focus:ring ring-red-300 ring-opacity-50':
              computedErrs.state.value,
            'focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50':
              !computedErrs.state.value,
          }"
        />
      </div>
    </label>
    <ErrorList :errors="computedErrs.messages.value" />
  </div>
</template>

<script setup lang="ts">
import type { ValidationRule } from "@/composables/formValidation";
import ErrorList from "@/components/fields/ErrorList.vue";
import useComputedErrors from "~/composables/errors";

const model = defineModel<number>({ default: 0, required: true });

const props = withDefaults(
  defineProps<{
    label: string;
    placeholder: string;

    errors?: string[];
    validationRules?: ValidationRule<number>[];
  }>(),
  {
    errors: () => [],
    validationRules: () => [],
  },
);

const computedErrs = useComputedErrors<number>(
  model,
  props.validationRules,
  props.errors,
);
</script>
