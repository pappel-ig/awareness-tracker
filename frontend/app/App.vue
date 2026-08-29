<script setup lang="ts">
</script>
<template>
  <div>
    <UApp>
      <UContainer class="max-w-3xl sm:mt-12">
        <UCard
          class="w-full p-1 sm:p-9 mb-3"
          :ui="{ root: 'ring-0 rounded-none shadow-none sm:ring sm:ring-default sm:rounded-md sm:shadow-sm' }"
        >
          <div class="text-xs text-primary tracking-wider uppercase flex flex-col sm:flex-row sm:items-center gap-0 sm:gap-1.5 font-extrabold mb-2">
            <span>Forschungsprojekt</span>
            <span class="hidden sm:inline">·</span>
            <span>Jonas Metzger</span>
            <span class="hidden sm:inline">·</span>
            <span>Security Awareness</span>
          </div>
          <h1 class="font-serif text-3xl sm:text-4xl text-black mb-5 tracking-tight">
            Umfrage Security Awareness
          </h1>
          <NuxtPage/>
        </UCard>
      </UContainer>
    </UApp>
  </div>
</template>
