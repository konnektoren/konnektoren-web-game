export default function routingInitializer() {
  let startTime = 0;

  return {
    onStart: () => {
      startTime = performance.now();
      document.querySelector(".routing .loading__message").textContent =
        "Loading...";
    },
    onProgress: ({ current, total }) => {
      const progressBar = document.querySelector(".routing .loading__progress");
      const loadingMessage = document.querySelector(
        ".routing .loading__message",
      );

      if (total) {
        const percentage = Math.round((current / total) * 100);
        progressBar.style.width = `${percentage}%`;
        loadingMessage.textContent = `Loading... ${percentage}%`;
      } else {
        loadingMessage.textContent = "Loading...";
      }
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    },
    onComplete: () => {
      document
        .querySelector(".routing .loading")
        .classList.add("loading--loaded");
    },
  };
}
