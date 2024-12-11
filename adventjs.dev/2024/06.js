// https://adventjs.dev/en/challenges/2024/6

/** @param {string[]} box
 *  @returns {boolean} True if the gift is inside the box
 */
function inBox(box) {
  return box
    .slice(1, box.length - 1)
    .some(line => Boolean(line.match(/^#\s*\*\s*#$/)));
}
