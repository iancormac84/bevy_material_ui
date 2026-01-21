import init from "./showcase_minimal.js";

const overlay = document.querySelector(".overlay");
const hint = document.querySelector(".hint");

const resizeCanvasToViewport = () => {
  const canvas = document.querySelector("canvas");
  if (!canvas) {
    return;
  }
  const width = window.innerWidth;
  const height = window.innerHeight;
  if (canvas.width !== width) {
    canvas.width = width;
  }
  if (canvas.height !== height) {
    canvas.height = height;
  }
  canvas.style.width = "100%";
  canvas.style.height = "100%";
};

init()
  .then(() => {
    resizeCanvasToViewport();
    window.addEventListener("resize", resizeCanvasToViewport);
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
