// https://adventjs.dev/challenges/2024/23

/**
 * @param {number[]} nums - List of integers.
 * @returns {number[]} - List of missing numbers.
 */
function findMissingNumbers(nums) {
  const current = new Set(nums);

  const n = Math.max(...nums);
  const allNumbers = [...Array.from({ length: n }, (_, i) => i + 1)];
  const expected = new Set(allNumbers);

  return [...expected.difference(current)];
}
