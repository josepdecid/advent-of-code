// https://adventjs.dev/challenges/2024/22

/**
 * @param {string[]} gifts - List of unique gifts.
 * @returns {string[][]} - All possible combinations of gifts, sorted by length.
 */
function generateGiftSets(gifts) {
  const combinations = [];

  function generateCombinationsOfSize(start, size, current) {
    if (current.length === size) {
      combinations.push([...current]);
      return;
    }
    
    for (let i = start; i < gifts.length; i++) {
      current.push(gifts[i]);
      generateCombinationsOfSize(i + 1, size, current);
      current.pop();
    }
  }

  for (let size = 1; size <= gifts.length; size++) {
    generateCombinationsOfSize(0, size, []);
  }

  return combinations;
}
    
