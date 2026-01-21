import init from "./showcase_minimal.js";

const overlay = document.querySelector(".overlay");
const hint = document.querySelector(".hint");

init()
  .then(() => {
    if (overlay) {
      overlay.remove();
    }
  })
  .catch((err) => {
    console.error("Failed to start showcase:", err);
    if (hint) {
      hint.textContent = "Failed to load WebGL demo. See console for details.";
    }
  });
