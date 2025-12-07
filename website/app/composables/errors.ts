import type { ValidationRule } from "@/composables/formValidation";
import { computed, type Ref } from "vue";

const NO_MESSAGE = "__ERROR_STATE__";

export default function useComputedErrors<T>(
  modelValue: Ref<T>,
  rules: ValidationRule<T>[],
  defaultErrors: string[],
) {
  const raw = computed<string[]>(() => {
    const errors = [] as string[];
    errors.push(...defaultErrors);
    const value = modelValue.value;

    for (const rule of rules) {
      const rsp = rule(value);
      if (typeof rsp === "boolean" && rsp == true) {
        errors.push(NO_MESSAGE);
      } else if (typeof rsp === "string") {
        errors.push(rsp);
      }
    }

    return errors;
  });

  const state = computed<boolean>(() => raw.value.length > 0);

  const messages = computed<string[]>(() =>
    raw.value.filter((v) => v !== NO_MESSAGE),
  );

  return {
    raw,
    state,
    messages,
  };
}
