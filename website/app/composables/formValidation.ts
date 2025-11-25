export type ValidationRule<T> = (value: T) => boolean | string;

export default function useFormValidation(_form: HTMLFormElement) {}
