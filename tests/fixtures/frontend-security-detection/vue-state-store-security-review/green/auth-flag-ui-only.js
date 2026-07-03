// components/UserAdmin.vue <script setup>
import { computed } from 'vue'
import { useUserStore } from '@/stores/user'
import api from '@/api'

const userStore = useUserStore()

// Client flag only controls whether the delete button renders/enables in the
// UI. It is never used to gate whether the mutating request is sent — the
// server independently re-verifies the caller's permissions on every request
// and returns 403 if the caller is not actually an admin.
const canShowDeleteButton = computed(() => userStore.isAdmin)

async function deleteUser(id) {
  await api.delete(`/users/${id}`)
}

export { canShowDeleteButton, deleteUser }
