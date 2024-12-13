/**
 * @param {number[]} reindeer
 * @param {number[]} stables
 * @returns {number}
 */
function minMovesToStables(reindeer, stables) {
  reindeer.sort((a, b) => a - b);
  stables.sort((a, b) => a - b);

  return reindeer.reduce(
    (acc, val, idx) => acc + Math.abs(val - stables[idx]),
    0
  );
}
