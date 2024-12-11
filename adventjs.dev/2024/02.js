// https://adventjs.dev/en/challenges/2024/2

/**
 * @param {string[]} names - Array of names to frame
 * @returns {string} The framed names
 */
function createFrame(names) {
  const maxLength = Math.max(...names.map((x) => x.length));

  const edgeLines = "*".repeat(maxLength + 4);
  const mappedNames = names.map((name) => (
    "* " + name + " ".repeat(maxLength - name.length) + " *"
  )).join("\n");

  return edgeLines + "\n" + mappedNames + "\n" + edgeLines;
}
