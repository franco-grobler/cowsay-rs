<template>
  <footer
    class="flex flex-row items-center justify-between whitespace-nowrap mt-12 py-4"
  >
    <div>This is an unlicensed project.</div>
    <div class="flex flex-row justify-end-safe w-full h-full">
      <CurrencyEuroIcon />
    </div>
  </footer>
</template>

<script setup lang="ts">
import { CurrencyEuroIcon } from "@heroicons/vue/16/solid";
</script>
