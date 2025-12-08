<template>
  <div class="font-display bg-background-light text-text-primary">
    <!-- class="font-display bg-background-light dark:bg-background-dark text-text-primary dark:text-text-primary-dark" -->
    <div
      class="relative flex min-h-screen w-full flex-col group/design-root overflow-x-hidden"
    >
      <div class="layout-container flex h-full grow flex-col">
        <div class="flex flex-1 justify-center py-5">
          <div
            class="layout-content-container flex flex-col max-w-[1020px] flex-1 px-4 sm:px-8"
          >
            <AppHeader />

            <NuxtPage />

            <AppFooter />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import AppFooter from "@/components/layout/AppFooter.vue";
import AppHeader from "@/components/layout/AppHeader.vue";
import { useHead, useSeoMeta } from "#app";

const logo = new URL("@/assets/logo.svg", import.meta.url).href;

const title = "Cowsay-RS";
const description = "Command line utility for talking cows.";
useSeoMeta({
  description: description,
  ogTitle: title,
  ogDescription: description,
  ogImage: logo,
  ogUrl: import.meta.env.BASE_URL,
  twitterTitle: title,
  twitterDescription: description,
  twitterImage: logo,
  twitterCard: "summary",
});

useHead({
  htmlAttrs: {
    lang: "en",
  },
  link: [
    {
      rel: "icon",
      type: "image/x-icon",
      href: "/favicon.ico",
    },
  ],
});
</script>
