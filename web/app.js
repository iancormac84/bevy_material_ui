import init from "./showcase_minimal.js";

init().catch((err) => {
  console.error("Failed to start showcase:", err);
  const hint = document.querySelector(".hint");
  if (hint) {
    hint.textContent = "Failed to load WebGL demo. See console for details.";
  }
});
