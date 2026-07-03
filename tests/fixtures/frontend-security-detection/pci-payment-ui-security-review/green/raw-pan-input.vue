<template>
  <form @submit.prevent="submitPayment">
    <!-- CardNumberElement renders inside a Stripe-controlled iframe; the
         merchant page never sees or touches the raw PAN. -->
    <div id="card-number-element"></div>
    <div id="card-cvc-element"></div>
    <button type="submit">Pay</button>
  </form>
</template>

<script>
import { CardNumberElement, CardCvcElement } from '@stripe/react-stripe-js';

export default {
  mounted() {
    const elements = this.stripe.elements();
    this.cardNumberElement = elements.create('cardNumber');
    this.cardNumberElement.mount('#card-number-element');
    this.cardCvcElement = elements.create('cardCvc');
    this.cardCvcElement.mount('#card-cvc-element');
  },
  methods: {
    async submitPayment() {
      const { token } = await this.stripe.createToken(this.cardNumberElement);
      this.$emit('submit', { token: token.id });
    },
  },
};
</script>
