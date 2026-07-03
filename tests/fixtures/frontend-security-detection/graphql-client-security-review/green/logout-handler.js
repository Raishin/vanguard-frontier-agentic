import { navigate } from "./router";
import client from "./apollo-client-setup";

async function handleLogout() {
  await logoutAPI();
  await client.resetStore();
  navigate("/login");
}

export default handleLogout;
