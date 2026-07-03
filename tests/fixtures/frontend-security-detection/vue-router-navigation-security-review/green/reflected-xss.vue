<template>
  <!-- Search results page. The raw query string is sanitized with DOMPurify
       on this exact path before it is ever handed to v-html. -->
  <div v-html="sanitizedQuery"></div>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import DOMPurify from 'dompurify'

const route = useRoute()
const sanitizedQuery = computed(() => DOMPurify.sanitize(route.query.q))
</script>
