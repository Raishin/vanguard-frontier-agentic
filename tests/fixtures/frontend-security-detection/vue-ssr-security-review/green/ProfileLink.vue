<script setup>
import { defineProps, computed } from 'vue'
const props = defineProps({ userProvidedUrl: String })

const ALLOWED_SCHEMES = ['http:', 'https:', 'mailto:']
function safeHref(url) {
  try {
    const parsed = new URL(url, window.location.origin)
    return ALLOWED_SCHEMES.includes(parsed.protocol) ? url : '#'
  } catch {
    return '#'
  }
}

</script>

<template>
  <a :href="safeHref(userProvidedUrl)">Visit my website</a>
</template>
