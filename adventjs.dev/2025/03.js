/**
 * @param {number} size - The size of the gift
 * @param {string} symbol - The symbol to draw
 * @returns {string} The gift drawn
 */
function drawGift(size, symbol) {
  if (size < 2) return "";

  const borderLine = Array(size).fill(symbol).join("");
  const centerLine = symbol + Array(size - 2).fill(" ").join("") + symbol;

  return [
    borderLine,
    ...Array(size - 2).fill(centerLine),
    borderLine
  ].join("\n");
}