export default function appInitializer() {
  let startTime = 0;

  const getElement = (selector) => document.querySelector(selector);

  const updateElement = (selector, action) => {
    const element = getElement(selector);
    if (element) action(element);
  };

  return {
    onStart: () => {
      console.log("onStart");
      startTime = performance.now();
      updateElement("#app .loading__message", (el) => {
        el.textContent = "Loading...";
      });
    },
    onProgress: ({ current, total }) => {
      console.log("current", current);
      if (total) {
        const percentage = Math.round((current / total) * 100);
        updateElement("#app .loading__progress", (el) => {
          el.style.width = `${percentage}%`;
        });
        updateElement("#app .loading__message", (el) => {
          el.textContent = `Loading... ${percentage}%`;
        });
      } else {
        updateElement("#app .loading__message", (el) => {
          el.textContent = "Loading...";
        });
      }
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    },
    onComplete: () => {
      updateElement("#app .loading", (el) => {
        el.classList.add("loading--loaded");
      });
    },
  };
}
