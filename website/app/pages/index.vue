<template>
  <div class="text-center">
    <IntroductionView />
    <AboutView class="my-6" />
    <PlaygroundView id="playground-section" />
  </div>
</template>

<script setup lang="ts">
import IntroductionView from "@/components/views/IntroductionView.vue";
import AboutView from "@/components/views/AboutView.vue";
import PlaygroundView from "@/components/views/PlaygroundView.vue";
</script>
