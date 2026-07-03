<template>
  <!-- profileLink comes straight from the user's editable profile "website"
       field; nothing stops it from being javascript:alert(document.cookie)
       or data:text/html,<script>... -->
  <router-link :to="profileLink">Visit my site</router-link>
</template>

<script setup>
defineProps({ profileLink: String })
</script>
