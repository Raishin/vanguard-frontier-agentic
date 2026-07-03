import { ApolloClient, InMemoryCache } from "@apollo/client";

// Devtools left wired up unconditionally -- ships to production builds too,
// exposing full schema introspection and the normalized cache inspector to
// anyone who opens the Apollo Client Devtools browser extension.
const client = new ApolloClient({
  uri: "/graphql",
  cache: new InMemoryCache(),
  connectToDevTools: true
});

export default client;
