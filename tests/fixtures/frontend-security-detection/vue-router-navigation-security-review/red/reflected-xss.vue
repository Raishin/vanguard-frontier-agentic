<template>
  <!-- Search results page. The query string "q" is echoed back verbatim as
       markup, so ?q=<img src=x onerror=alert(document.cookie)> executes. -->
  <div v-html="route.query.q"></div>
</template>

<script setup>
import { useRoute } from 'vue-router'
const route = useRoute()
</script>
