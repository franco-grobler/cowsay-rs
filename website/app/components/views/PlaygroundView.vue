<template>
  <section>
    <SectionTitle>Try cowsay-rs in your browser!</SectionTitle>
    <p class="my-6">
      WebAssembly allows for JavaScript bindings to the Rust application!
    </p>
    <div class="flex flex-row w-full justify-between gap-5">
      <form class="flex flex-col gap-3 max-w-[200px]">
        <TextField
          v-model="eyes"
          label="Eyes"
          placeholder="Eye characters"
          :validation-rules="[twoLetterRule]"
        />
        <TextField
          v-model="tongue"
          label="Tongue"
          placeholder="Tongue characters"
          :validation-rules="[twoLetterRule]"
        />
        <TextArea v-model="text" label="Text" rows="2" />
        <SelectField v-model="file" label="Cowfile" :items="availableCows" />
        <SwitchField v-model="wrap" label="Wrap" />
        <NumericField
          v-model="wrapColumn"
          label="Wrap width"
          placeholder="Wrap column"
        />
      </form>
      <div class="flex flex-col grow gap-3 min-w-[300px] h-full">
        <TextArea
          v-model="cow"
          label="What the cow has to say:"
          class="h-full w-full overflow-hidden"
          textarea-class="overflow-hidden"
          readonly
          :rows="textRows"
          style="font-family: monospace; field-sizing: content"
        />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import TextField from "@/components/fields/TextField.vue";
import TextArea from "@/components/fields/TextArea.vue";
import SelectField from "@/components/fields/SelectField.vue";
import SwitchField from "@/components/fields/SwitchField.vue";
import NumericField from "@/components/fields/NumericField.vue";
import SectionTitle from "@/components/titles/SectionTitle.vue";
import { computed, ref } from "vue";
import useCowsay, { type CowsayOptions } from "@/composables/cowsay";

const eyes = ref<string>("oo");
const tongue = ref<string>("U ");
const text = ref<string>("Hello, World!");
const file = ref<string>("default");
const wrap = ref<boolean>(true);
const wrapColumn = ref<number>(40);

const twoLetterRule = (value: string) =>
  value.length > 2 ? "Let's now make weird cows." : false;

const cowData = computed<CowsayOptions>(() => {
  return {
    eyes: eyes.value.padEnd(2, " "),
    tongue: tongue.value.padEnd(2, " "),
    file: file.value,
    wrap: wrap.value,
    wrap_column: wrapColumn.value.toString(),
  };
});
const cow = computed<string>(() =>
  getCowText(text.value.trim(), cowData.value),
);

const textRows = computed<number>(() =>
  Math.max(8, cow.value.split("\n").filter((v) => v.trim().length > 0).length),
);

const availableCows = computed<string[]>(() => getAvailableCows());

const { getAvailableCows, getCowText } = useCowsay();
</script>
