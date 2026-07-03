import { SetContextLink } from "@apollo/client/link/context";

const withToken = new SetContextLink(async (prevContext, operation) => {
  const token = await AsyncTokenLookup();
  return {
    headers: {
      ...prevContext.headers,
      authorization: `Bearer ${token}`
    }
  };
});

export default withToken;
