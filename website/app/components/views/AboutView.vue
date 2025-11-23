<template>
  <section class="text-center">
    <SectionTitle>About is project</SectionTitle>
    <p>
      The original cowsay was released in 1999 - the year I was born - by Tony
      Monroe. Tony maintaiend the project until 2016, when he released the final
      version. After this, the
      <a href="https://github.com/cowsay-org">cowsay-org</a> was formed to
      maintain the project.
    </p>
    <p>
      There has been a trend to rewrite everything in Rust over the last few
      years, and this project is just riding the hype. 🦀
    </p>
    <p>
      The slightly more utilitarian reason is to build something in Rust without
      the risk of breaking something important. A cli printing ASCII cows is
      basically as low risk as you can get, while still having some, admittedly
      questionable, use.
    </p>
  </section>
</template>

<script setup lang="ts">
import SectionTitle from "@/components/titles/SectionTitle.vue";
</script>
