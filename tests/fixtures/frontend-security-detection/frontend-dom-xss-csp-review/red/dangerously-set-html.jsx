// Renders a user profile bio fetched from the public API.
function ProfileBio({ userBio }) {
  // userBio is attacker-controlled (any user can set their own bio) and is
  // injected as raw HTML with no sanitizer pass.
  return <div dangerouslySetInnerHTML={{ __html: userBio }} />;
}
