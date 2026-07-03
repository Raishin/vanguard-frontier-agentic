<script setup>
import DOMPurify from 'dompurify'
const { data: comment } = await useAsyncData('comment',
  () => $fetch(`/api/comments/${id}`)
)
const safeBody = computed(() => DOMPurify.sanitize(comment.value?.body ?? ''))
</script>

<template>
  <div v-html="safeBody" /> <!-- sanitizer explicitly applied on this exact path -->
</template>
