const disallowedURLs = /https:\/\/discord\.com\/api\/v\d+\/science/;

const originalOpen = XMLHttpRequest.prototype.open;

Object.defineProperty(XMLHttpRequest.prototype, "open", {
  value: function (method: string, url: string) {
    if (method === "POST" && disallowedURLs.test(url)) {
      throw new Error(`XMLHttpRequest to "${url}" with POST method is blocked`);
    }

    return originalOpen.apply(this, arguments as any);
  },
});

document.addEventListener("DOMContentLoaded", () => {
  const styleElement = document.createElement("style");

  const rules = [
    "[class|=listItem]:has([data-list-item-id=guildsnav___app-download-button]) { display: none; }",
    "[class|=listItem]:has(+ [class|=listItem] [data-list-item-id=guildsnav___app-download-button]) { display: none; }",
  ];

  styleElement.innerHTML = rules.join("\n");
  document.head.appendChild(styleElement);
});