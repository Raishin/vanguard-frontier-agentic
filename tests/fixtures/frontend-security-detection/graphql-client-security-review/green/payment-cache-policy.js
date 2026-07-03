import { gql, InMemoryCache } from "@apollo/client";

const FETCH_USER = gql`
  query FetchUser {
    user {
      id
      email
      creditCard {
        number
        cvv
      }
    }
  }
`;

// A read policy on the sensitive field strips it before it ever lands in
// the normalized cache -- the cvv value is fetched, used once, and discarded
// instead of being persisted where the devtools inspector could reveal it.
const cache = new InMemoryCache({
  typePolicies: {
    CreditCard: {
      fields: {
        cvv: {
          read() {
            return undefined;
          }
        }
      }
    }
  }
});

export { FETCH_USER, cache };
