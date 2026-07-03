import { ApolloClient, InMemoryCache } from "@apollo/client";

// Devtools are gated behind the environment check -- only wired up when
// actually running a development build, never in production.
const client = new ApolloClient({
  uri: "/graphql",
  cache: new InMemoryCache(),
  connectToDevTools: process.env.NODE_ENV === "development"
});

export default client;
