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

// No typePolicies at all -- every field returned by FetchUser, including
// the raw card number and cvv, is written into the normalized cache as-is
// and is fully visible in the Apollo Client Devtools cache inspector.
const cache = new InMemoryCache();

export { FETCH_USER, cache };
