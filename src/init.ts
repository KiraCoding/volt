const disallowedURLs = /https:\/\/discord\.com\/api\/v\d+\/science/;

const originalOpen = XMLHttpRequest.prototype.open;

Object.defineProperty(XMLHttpRequest.prototype, "open", {
  value: function (method, url) {
    if (method === "POST" && disallowedURLs.test(url)) {
      throw new Error(`XMLHttpRequest to "${url}" with POST method is blocked`);
    }

    return originalOpen.apply(this, arguments);
  },
});

document.addEventListener("DOMContentLoaded", function () {
  const styleElement = document.createElement("style");
  styleElement.innerHTML = `
  [class|=listItem]:has([data-list-item-id=guildsnav___app-download-button]),
  [class|=listItem]:has(+ [class|=listItem] [data-list-item-id=guildsnav___app-download-button]) {
      display: none;
  }`;
  document.head.appendChild(styleElement);
});
