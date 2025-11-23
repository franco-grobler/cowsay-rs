<template>
  <body class="font-display bg-background-light text-text-primary">
    <div
      class="relative flex min-h-screen w-full flex-col group/design-root overflow-x-hidden"
    >
      <div class="layout-container flex h-full grow flex-col">
        <div class="flex flex-1 justify-center py-5">
          <div
            class="layout-content-container flex flex-col max-w-[960px] flex-1 px-4 sm:px-8"
          >
            <AppHeader />

            <NuxtPage />

            <AppFooter />
          </div>
        </div>
      </div>
    </div>
  </body>
</template>

<script setup lang="ts">
import { useHead, useSeoMeta } from "#app";
import AppHeader from "@/components/layout/AppHeader.vue";
import AppFooter from "@/components/layout/AppFooter.vue";

useSeoMeta({
  description: "Command line utility for talking cows.",
  ogTitle: "Cowsay-RS",
  ogDescription: "[og:description]",
  ogImage: "[og:image]",
  ogUrl: "[og:url]",
  twitterTitle: "[twitter:title]",
  twitterDescription: "[twitter:description]",
  twitterImage: "[twitter:image]",
  twitterCard: "summary",
});

useHead({
  htmlAttrs: {
    lang: "en",
  },
  link: [
    {
      rel: "icon",
      type: "image/webp",
      href: "/favicon.ico",
    },
  ],
});
</script>
