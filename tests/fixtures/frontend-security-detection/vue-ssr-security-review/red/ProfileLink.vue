<script setup>
import { defineProps } from 'vue'
// userProvidedUrl is taken directly from a query param (?website=javascript:alert(document.cookie))
const props = defineProps({ userProvidedUrl: String })
</script>

<template>
  <a :href="userProvidedUrl">Visit my website</a>
</template>
