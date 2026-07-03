import { ApolloClient, InMemoryCache, HttpLink } from "@apollo/client";

const httpLink = new HttpLink({ uri: "/graphql" });

// No PersistedQueryLink in the chain -- any client (or anyone replaying a
// captured request with modifications) can send an arbitrary, arbitrarily
// deep/aliased query document straight to the server with no allowlist.
const client = new ApolloClient({
  link: httpLink,
  cache: new InMemoryCache()
});

export default client;
