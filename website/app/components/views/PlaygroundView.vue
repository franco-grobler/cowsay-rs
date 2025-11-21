<template>
  <section>
    <SectionTitle>Try cowsay-rs in your browser!</SectionTitle>
    <p class="my-6">
      WebAssembly allows for JavaScript bindings to the Rust application!
    </p>
    <form>
      <TextField v-model="cowData.eyes" label="Eyes" />
    </form>
    {{ cowData }}
    <div class="flex flex-row justify-between gap-5 my-4">
      <DefaultBtn>Try it live</DefaultBtn>
      <DefaultBtn>Get started</DefaultBtn>
    </div>
  </section>
</template>

<script setup lang="ts">
import DefaultBtn from "@/components/buttons/DefaultBtn.vue";
import TextField from "@/components/fields/TextField.vue";
import SectionTitle from "@/components/titles/SectionTitle.vue";
import { ref } from "vue";

const cowData = ref<{ eyes?: string }>({});
</script>
