import { navigate } from "./router";

async function handleLogout() {
  await logoutAPI();
  // Normalized cache is never cleared here -- the previous user's queries,
  // including anything fetched into the shared InMemoryCache, remain
  // resolvable by the next session on this device (shared machine, browser
  // profile switch, or a subsequent user logging in without a full reload).
  navigate("/login");
}

export default handleLogout;
