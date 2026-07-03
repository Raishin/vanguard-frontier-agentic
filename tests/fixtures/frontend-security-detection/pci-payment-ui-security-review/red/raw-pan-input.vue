<template>
  <form @submit.prevent="submitPayment">
    <label for="cardnumber">Card number</label>
    <input type="text" id="cardnumber" v-model="cardNumber" maxlength="19" />
    <label for="cvv">CVV</label>
    <input type="text" id="cvv" v-model="cvv" maxlength="4" />
    <button type="submit">Pay</button>
  </form>
</template>

<script>
// Raw PAN collection in a self-controlled <input> — the merchant app JS
// reads .value directly, so the full card number and CVV pass through
// first-party memory/DOM instead of staying inside a Stripe iframe.
export default {
  data() {
    return { cardNumber: '', cvv: '' };
  },
  methods: {
    submitPayment() {
      const pan = document.getElementById('cardnumber').value;
      console.log('collected raw PAN from self-controlled input', pan);
      this.$emit('submit', { pan, cvv: this.cvv });
    },
  },
};
</script>
