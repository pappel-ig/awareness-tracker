<script setup lang="ts">

</script>

<template>
    <UCard class="w-full rounded-md shadow-sm p-9 mb-3">
      <div class="text-xs text-primary tracking-wider uppercase flex items-center gap-1.5 font-extrabold mb-2">
        <span>Forschungsprojekt</span>
        <span>·</span>
        <span>Jonas Metzger</span>
        <span>·</span>
        <span>Security Awareness</span>
      </div>
      <h1 class="font-serif text-3xl sm:text-4xl text-black mb-5 tracking-tight">
        Umfrage Security Awareness
      </h1>
      <p class="text-black text-md leading-relaxed mb-6">
        Vielen Dank das du bei der Umfrage mitmachst. Nachfolgend werden dir ein paar Fragen bzgl. deiner Security Awareness gestellt.
        Dabei werden auch Daten erhoben, diese kannst du jedoch im Anschluss der Umfrage einsehen. Wenn du diese gelöscht haben möchtest
        kannst du dich an mich wenden.
        <br><br>
        Die Umfrage kannst du auch an mehreren Geräten durchführen, dafür die E-Mail einfach auf den anderen Geräten öffnen.
      </p>
      <div class="text-xs text-stone-500 font-sans mb-4">
        Die Umfrage dauert ca. 5-10 Minuten
      </div>
      <UButton
          variant="outline"
          size="md"
          trailing-icon="i-heroicons-arrow-right-20-solid"
          class="rounded-sm px-4 py-2"
      >
        Starten
      </UButton>
    </UCard>
</template>

<style scoped>

</style>