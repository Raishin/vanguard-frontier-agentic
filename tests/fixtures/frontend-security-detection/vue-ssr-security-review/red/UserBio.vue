<script setup>
import { defineProps } from 'vue'
const props = defineProps({ userBio: String })
</script>

<template>
  <!-- userBio comes straight from a user profile field with no sanitization -->
  <div v-html="userBio"></div>
</template>
