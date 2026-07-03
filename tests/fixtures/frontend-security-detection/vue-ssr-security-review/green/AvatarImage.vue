<script setup>
import { defineProps } from 'vue'
const props = defineProps({ userProvidedImageUrl: String })

const ALLOWED_SCHEMES = ['http:', 'https:']
function safeSrc(url) {
  try {
    const parsed = new URL(url, window.location.origin)
    return ALLOWED_SCHEMES.includes(parsed.protocol) ? url : '/img/placeholder.png'
  } catch {
    return '/img/placeholder.png'
  }
}
</script>

<template>
  <img :src="safeSrc(userProvidedImageUrl)" alt="avatar" />
</template>
