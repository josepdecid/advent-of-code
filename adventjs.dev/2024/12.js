// https://adventjs.dev/en/challenges/2024/12

/** @param {string} ornaments
 * @return {number} - The price of the tree
 */
function calculatePrice(ornaments) {
  const itemToValue = { "*": 1, "o": 5, "^": 10, "#": 50, "@": 100 };

  let result = 0;
  for (let i = 0; i < ornaments.length; ++i) {
    let value = itemToValue[ornaments[i]];
    if (!value) {
      return undefined;
    }

    const nextValue = itemToValue[ornaments[i + 1]];
    result += (nextValue > value) ? -value : value;
  }

  return result;
}