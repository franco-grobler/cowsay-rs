<template>
  <section class="text-center">
    <PageTitle class="px-5 text-center"
      >A friendly cowsay for the Rust era.</PageTitle
    >
    <p class="my-6 mx-auto w-2/3">
      cowsay-rs is a modern, fast, and fun rewrite of the classic cowsay command
      line program, built with Rust and WebAssembly bindings for browser
      support.
    </p>
    <div class="flex flex-row justify-center gap-5 my-4">
      <RouterLink to="#playground-section">
        <DefaultBtn>Try it live</DefaultBtn>
      </RouterLink>
      <RouterLink to="#playground-section">
        <DefaultBtn>Get started</DefaultBtn>
      </RouterLink>
    </div>
  </section>
</template>

<script setup lang="ts">
import DefaultBtn from "@/components/buttons/DefaultBtn.vue";
import PageTitle from "@/components/titles/PageTitle.vue";
</script>
