# Hosted Fields and the Tokenization Boundary

Use this reference when reviewing card-entry form markup, or investigating the `raw-pan-input`, `card-data-persistence`, or `self-posted-card-data` defect classes.

## What people get wrong

The naive assumption is:

> "As long as I send the card data over HTTPS to my own server, it's protected in transit, so a plain `<input>` for the card number is fine."

Wrong for PCI-DSS purposes. Transport encryption protects data in transit between the browser and the first-party server, but it does nothing to prevent the merchant's own JavaScript — or a compromised/malicious third-party script sharing the page — from reading the raw PAN/CVV the instant it sits in a DOM `<input>` or in memory as a plain JS value. The entire architectural point of Stripe's hosted fields is to remove that reachability, not merely to encrypt what is already reachable.

## Officially grounded tokenization flow (Context7 Stripe documentation)

- `stripe.createToken(cardElement)` converts data collected by Stripe Elements into a **single-use token**, passed securely to the server — the merchant's own code never handles the raw values that produced the token.
- Raw card data (PAN, CVV) collected by `CardNumberElement`, `CardExpiryElement`, and `CardCVCElement` stays **inside iframes controlled by Stripe**, never accessible to merchant JavaScript.
- `stripe.confirmPayment()` and `stripe.confirmCardPayment()` tokenize directly as part of confirming the PaymentIntent, without ever exposing raw values to the page.
- The merchant receives only the token, or the PaymentIntent/PaymentMethod confirmation result — never the raw card data itself.
- **Key architectural guarantee:** Elements render inside sandboxed iframes; merchant code never sees or touches the PAN or CVV. The browser's same-origin policy prevents scripts on the merchant page — including the merchant's own code and any co-located third-party script — from reading iframe content.

## Defect class 1: `raw-pan-input`

Raw PAN collection in a self-controlled `<input>` element.

- **Dangerous:** `<input type="text" id="cardnumber" />` with application JS reading `.value` to obtain the card number. The moment card-number digits exist as a plain string in application-reachable memory or the DOM, they are reachable by merchant JS, any injected script, and any browser extension with page access.
- **Safe:** `<CardNumberElement />` (iframe-rendered by Stripe; no app JS access to the value inside).
- **Verification targets:** Grep for `<input` elements with `type="text"`/`type="number"`/`name`/`id` attributes suggestive of card fields (`card`, `pan`, `cardnumber`, `ccnum`), and confirm whether the surrounding component imports and uses `CardNumberElement`/`PaymentElement` instead, or reads `.value` directly from a raw input.

## Defect class 2: `missing-iframe-isolation`

No iframe/hosted-field isolation for card-data collection generally (PAN, CVV, *or* expiry).

- **Dangerous:** Manual `<input>` fields for PAN, CVV, and/or expiry with no iframe sandboxing — a homegrown card form.
- **Safe:** `<CardNumberElement />`, `<CardExpiryElement />`, `<CardCVCElement />`, or the unified `<PaymentElement />`, all of which render inside a sandboxed iframe boundary that the merchant page cannot reach into.
- This is the broader structural finding that `raw-pan-input` is one specific instance of — flag it whenever *any* of the three card fields (not just the PAN) is collected outside a hosted field.

## Defect class 3: `card-data-persistence`

Card data persisted to a client-side store, `localStorage`/`sessionStorage`, analytics, or logs.

- **Dangerous:** `localStorage.setItem('card', {pan, cvv, exp})`, a Vuex/Redux/Pinia `store` action that commits raw card fields to persisted state, or an analytics event (`analytics.track('checkout_attempt', {cardNumber, cvv})`) that includes cardholder data fields.
- **Safe:** Persist only the tokenized `paymentMethod` ID (e.g., `store.commit('setPaymentMethod', paymentMethod.id)`) — never raw PAN, CVV, or full cardholder data.
- **Verification targets:** Grep for `localStorage.setItem`/`sessionStorage.setItem` calls, and for `store`/state-management writes and analytics/logging calls, in files reachable from the payment form. For each match, inspect the literal object shape being persisted for PAN- or CVV-shaped fields, even under generic key names — a field literally named `PAN` or `CVV`, or one holding a 13–19 digit numeric string or 3–4 digit CVV pattern, is the signal to trace, regardless of the variable name chosen.

## Defect class 4: `self-posted-card-data`

Raw card fields POSTed to a first-party endpoint.

- **Dangerous:** `fetch('/api/pay', {body: {pan, cvv, exp}})` — raw card fields constructed directly into a POST body and sent to the merchant's own `/api/pay` first-party endpoint, with no tokenization step beforehand.
- **Safe:** `stripe.createToken(cardElement).then(token => fetch('/api/pay', {body: {token: token.id}}))` — the Stripe API tokenizes first; only the resulting token is POSTed to the first-party endpoint.
- **Verification targets:** Grep for `fetch(`/`XMLHttpRequest`/form `action=` submissions targeting a first-party API path from payment-form components. For each, trace the request body's construction backward: does it reference raw card fields (`pan`, `cardNumber`, `cvv`, `exp`), or does it reference a token/PaymentIntent/PaymentMethod object produced by a preceding Stripe API call?

## Minimal safe implementation pattern

```jsx
// Safe: hosted fields + tokenize-before-POST
import { CardNumberElement, CardExpiryElement, CardCVCElement, useStripe, useElements } from '@stripe/react-stripe-js'

function CheckoutForm() {
  const stripe = useStripe()
  const elements = useElements()

  async function handleSubmit(e) {
    e.preventDefault()
    const cardElement = elements.getElement(CardNumberElement)
    // Raw PAN/CVV never leave the Stripe iframe; only a token is produced.
    const { token } = await stripe.createToken(cardElement)
    // Only the token is persisted or sent to the first-party endpoint.
    await fetch('/api/pay', { method: 'POST', body: JSON.stringify({ token: token.id }) })
  }

  return (
    <form onSubmit={handleSubmit}>
      <CardNumberElement />
      <CardExpiryElement />
      <CardCVCElement />
    </form>
  )
}
```

Anti-pattern (do not approve):

```html
<!-- WRONG: raw PAN/CVV in a self-controlled input, read directly by app JS -->
<input type="text" id="cardnumber" />
<input type="text" id="cvv" />
<script>
  function submitPayment() {
    const pan = document.getElementById('cardnumber').value
    const cvv = document.getElementById('cvv').value
    localStorage.setItem('lastCard', JSON.stringify({ pan, cvv })) // card-data-persistence
    fetch('/api/pay', { method: 'POST', body: JSON.stringify({ pan, cvv }) }) // self-posted-card-data
  }
</script>
```

## Adversarial checklist

Before clearing a payment form as safe on the tokenization boundary, answer these:

- Is card number, expiry, and CVV collection fully delegated to `CardNumberElement`/`CardExpiryElement`/`CardCVCElement`/`PaymentElement`, or does any field use a manual `<input>`?
- Does any code path read `.value` from a manual card-shaped input?
- Does any `localStorage`/`sessionStorage`/store/analytics/log call persist an object containing PAN-, CVV-, or cardholder-data-shaped fields?
- Does the first-party POST body reference raw card fields, or does it reference only a Stripe token/PaymentIntent/PaymentMethod ID produced by a preceding `stripe.createToken`/`stripe.confirmPayment`/`stripe.confirmCardPayment` call?

If any answer reveals raw PAN/CVV reachable by merchant JS at rest, in a POST body, or in persisted state, the finding is HIGH and structural — report it even without a reproduced data-exfiltration incident.
