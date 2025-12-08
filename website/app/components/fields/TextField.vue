<template>
  <div class="text-left">
    <label class="block">
      <span>{{ props.label }}</span>
      <input
        v-model="model"
        :type="props.type"
        :placeholder="props.placeholder"
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
        :class="{
          'border-red-600 focus:ring ring-red-300 ring-opacity-50':
            computedErrs.state.value,
          'focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50':
            !computedErrs.state.value,
        }"
      />
    </label>
    <ErrorList :errors="computedErrs.messages.value" />
  </div>
</template>

<script setup lang="ts">
import ErrorList from "@/components/fields/ErrorList.vue";
import type { ValidationRule } from "@/composables/formValidation";
import useComputedErrors from "~/composables/errors";

const model = defineModel<string>({ default: "", required: true });

const props = withDefaults(
  defineProps<{
    label: string;
    placeholder: string;
    type?: HTMLInputElement["type"];

    errors?: string[];
    validationRules?: ValidationRule<string>[];
  }>(),
  {
    type: "text",
    errors: () => [],
    validationRules: () => [],
  },
);

const computedErrs = useComputedErrors(
  model,
  props.validationRules,
  props.errors,
);
</script>
