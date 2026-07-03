import { useState } from 'react';

// A hand-rolled checkout form with plain <input> elements for card number,
// expiry, and CVV. None of this is rendered inside a sandboxed iframe or
// Stripe-hosted field, so the merchant's own React tree has direct DOM
// access to every character of cardholder data as it is typed.
export function ManualCardForm({ onSubmit }) {
  const [card, setCard] = useState({ number: '', expiry: '', cvc: '' });

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit(card);
      }}
    >
      <input
        aria-label="Card number"
        value={card.number}
        onChange={(e) => setCard({ ...card, number: e.target.value })}
      />
      <input
        aria-label="Expiry"
        value={card.expiry}
        onChange={(e) => setCard({ ...card, expiry: e.target.value })}
      />
      <input
        aria-label="CVC"
        value={card.cvc}
        onChange={(e) => setCard({ ...card, cvc: e.target.value })}
      />
      <button type="submit">Pay</button>
    </form>
  );
}
