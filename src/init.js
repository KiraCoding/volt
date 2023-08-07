document.addEventListener('DOMContentLoaded', function () {
  // Create a new style element
  var styleElement = document.createElement('style');

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