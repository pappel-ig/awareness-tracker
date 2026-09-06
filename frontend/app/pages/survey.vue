<script setup lang="ts">
const route = useRoute()
const router = useRouter()

if (!route.query.token) {
  router.replace({ path: '/start' })
}

</script>

<template>
  <SurveyGroup>

  </SurveyGroup>
</template>

<style scoped>

</style>