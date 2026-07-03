// stores/user.js
import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => ({
    theme: 'dark',
    authToken: null,
    ssn: null,
    creditCardLast4: null,
  }),
  actions: {
    login(token, ssn, cc) {
      this.authToken = token
      this.ssn = ssn
      this.creditCardLast4 = cc
    },
  },
  persist: {
    pick: ['theme'],
    storage: sessionStorage,
  },
})
