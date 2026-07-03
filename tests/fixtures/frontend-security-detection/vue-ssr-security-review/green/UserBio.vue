<script setup>
import { defineProps } from 'vue'
const props = defineProps({ userBio: String })
</script>

<template>
  <!-- Vue's default text interpolation auto-escapes HTML entities; no raw markup is ever injected -->
  <div>{{ userBio }}</div>
</template>
