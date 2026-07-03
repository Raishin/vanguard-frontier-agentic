// Renders a user profile bio fetched from the public API.
function ProfileBio({ userBio }) {
  // JSX text interpolation auto-escapes the string; React renders it as a
  // text node, so markup in userBio can never execute.
  return <div>{userBio}</div>;
}
