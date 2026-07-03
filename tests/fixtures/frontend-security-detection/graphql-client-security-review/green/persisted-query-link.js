import { ApolloClient, InMemoryCache, HttpLink } from "@apollo/client";
import { PersistedQueryLink } from "@apollo/client/link/persisted-queries";

const httpLink = new HttpLink({ uri: "/graphql" });

const persistedQueryLink = new PersistedQueryLink({
  generatePersistedQueryIdsFromManifest: () =>
    import("./persisted-query-manifest.json")
});

// Only allowlisted, pre-registered query IDs from the build-time manifest
// can reach the server -- arbitrary client-authored documents are rejected.
const client = new ApolloClient({
  link: persistedQueryLink.concat(httpLink),
  cache: new InMemoryCache()
});

export default client;
