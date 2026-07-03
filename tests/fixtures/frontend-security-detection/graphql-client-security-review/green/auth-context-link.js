import { SetContextLink } from "@apollo/client/link/context";

const withToken = new SetContextLink(async (prevContext, operation) => {
  const token = await AsyncTokenLookup();
  const csrfToken = await getCsrfToken();
  return {
    headers: {
      ...prevContext.headers,
      authorization: `Bearer ${token}`,
      "x-csrf-token": csrfToken
    }
  };
});

export default withToken;
