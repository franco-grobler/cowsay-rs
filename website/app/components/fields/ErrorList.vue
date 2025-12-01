<template>
  <div v-if="hasErrors" class="pa-4 text-wrap inline-block">
    <ul class="text-red-400 list-disc list-inside mx-auto">
      <li v-for="(item, idx) in errors" :key="idx" class="pa-2 list-item">
        {{ item }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  errors: string[];
}>();

const hasErrors = computed<boolean>(() => props.errors.length > 0);
</script>
