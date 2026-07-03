// Checkout autosave persists only the tokenized paymentMethod id returned
// by Stripe — never the raw PAN/CVV/expiry — so the client-side store
// never holds cardholder data at all.
function autosaveCardDraft(paymentMethodId) {
  localStorage.setItem('checkoutDraft', JSON.stringify({ paymentMethodId }));
  analytics.track('checkout_draft_saved', { paymentMethodId });
}

export { autosaveCardDraft };
