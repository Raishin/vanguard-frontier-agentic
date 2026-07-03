// components/UserAdmin.vue <script setup>
import { useUserStore } from '@/stores/user'
import api from '@/api'

const userStore = useUserStore()

async function deleteUser(id) {
  // WRONG: userStore.isAdmin is a client-writable flag (persisted/hydrated
  // state); gating the mutating network call on it is not an authorization
  // boundary — a user can flip it in devtools/localStorage and fire the call.
  if (userStore.isAdmin) {
    await api.delete(`/users/${id}`)
  }
}
