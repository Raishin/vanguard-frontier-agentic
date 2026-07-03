<script setup>
import { defineProps } from 'vue'
// userProvidedImageUrl comes from an untrusted profile field submitted by the user
const props = defineProps({ userProvidedImageUrl: String })
</script>

<template>
  <img :src="userProvidedImageUrl" alt="avatar" />
</template>
