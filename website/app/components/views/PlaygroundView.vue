<template>
  <section>
    <SectionTitle>Try cowsay-rs in your browser!</SectionTitle>
    <p class="my-6">
      WebAssembly allows for JavaScript bindings to the Rust application!
    </p>
    <div class="flex flex-row w-full justify-between gap-5">
      <form class="flex flex-col gap-3">
        <TextField
          v-model="cowData.eyes"
          label="Eyes"
          placeholder="Eye characters"
        />
        <TextField
          v-model="cowData.tongue"
          label="Tongue"
          placeholder="Tongue characters"
        />
      </form>
      <div class="flex flex-col gap-3 min-w-[300px]">
        <TextField v-model="text" label="Text" placeholder="Hello, World!" />
        <TextArea v-model="template" label="Cowfile" rows="6" />
      </div>
      <div>
        <TextArea
          v-model="cow"
          label="What the cow has to say:"
          class="h-full"
        />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import TextField from "@/components/fields/TextField.vue";
import TextArea from "@/components/fields/TextArea.vue";
import SectionTitle from "@/components/titles/SectionTitle.vue";
import { ref } from "vue";

const cowData = ref<{ eyes?: string; tongue?: string }>({});
const template = ref<string>(`$the_cow = <<"EOC";
        $thoughts   ^__^
         $thoughts  ($eyes)\\_______
            (__)\\       )\\/\\
             $tongue ||----w |
                ||     ||
EOC`);
const cow = ref<string>("");
const text = ref<string>();
</script>
