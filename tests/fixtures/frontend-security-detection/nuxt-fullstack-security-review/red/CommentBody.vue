<script setup>
const { data: comment } = await useAsyncData('comment',
  () => $fetch(`/api/comments/${id}`)
)
</script>

<template>
  <div v-html="comment.body" /> <!-- comment.body is user-submitted, no sanitizer on this path -->
</template>
