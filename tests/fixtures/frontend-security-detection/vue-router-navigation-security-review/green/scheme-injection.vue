<template>
  <!-- profileLink is validated against an explicit scheme allowlist before
       it is ever bound, so javascript:/data: payloads are rejected up front. -->
  <a :href="safeProfileLink">Visit my site</a>
</template>

<script setup>
const ALLOWED_SCHEMES = ['http:', 'https:', 'mailto:']

const props = defineProps({ profileLink: String })
const safeProfileLink = (() => {
  try {
    const url = new URL(props.profileLink, window.location.origin)
    return ALLOWED_SCHEMES.includes(url.protocol) ? url.href : '#'
  } catch {
    return '#'
  }
})()
</script>
