
const disallowedURLs = [
  "https://discord.com/api/v9/science",
];

const originalOpen = XMLHttpRequest.prototype.open;

Object.defineProperty(XMLHttpRequest.prototype, "open", {
  value: function (method, url) {
    if (method === "POST" && disallowedURLs.includes(url)) {
      throw new Error(`XMLHttpRequest to "${url}" with POST method is blocked`);
    }

    return originalOpen.apply(this, arguments);
  },
});

document.addEventListener("DOMContentLoaded", function () {
  // Create a new style element
  var styleElement = document.createElement("style");

  // Set the CSS code as the content of the style element
  var cssCode = `
      /* Hide the download apps button */
      [class|=listItem]:has([data-list-item-id=guildsnav___app-download-button]),
      [class|=listItem]:has(+ [class|=listItem] [data-list-item-id=guildsnav___app-download-button]) {
          display: none;
      }
  `;
  styleElement.innerHTML = cssCode;

  // Append the style element to the head section of the HTML document
  document.head.appendChild(styleElement);
});