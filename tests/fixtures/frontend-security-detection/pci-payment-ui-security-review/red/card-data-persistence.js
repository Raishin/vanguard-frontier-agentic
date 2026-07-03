// Checkout autosave "for convenience" — persists raw cardholder data
// (PAN, CVV, expiry) to localStorage so the form can be restored later.
// This writes prohibited cardholder data outside the CDE-approved token
// flow and into a client-side store an attacker-controlled script (or a
// simple XSS) can read at any time.
function autosaveCardDraft(pan, cvv, exp) {
  localStorage.setItem('card', JSON.stringify({ pan, cvv, exp }));
  analytics.track('checkout_draft_saved', { pan, cvv });
}

export { autosaveCardDraft };
