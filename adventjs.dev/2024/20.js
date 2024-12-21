// https://adventjs.dev/challenges/2024/20

/**
 * @typedef {Record<string, number>} GiftsCount
 */

/**
 * @typedef {{ missing: GiftsCount, extra: GiftsCount }} Result
 */

/**
 * @param {string[]} received
 * @param {string[]} expected
 * @returns {Result}
 */
function fixGiftList(received, expected) {
  const counts = {};
  received.forEach((gift) => {
    counts[gift] = (counts[gift] || 0) + 1;
  });
  expected.forEach((gift) => {
    counts[gift] = (counts[gift] || 0) - 1;
  });

  const result = { extra: {}, missing: {} };
  Object.entries(counts).forEach(([gift, count]) => {
    if (count > 0) {
      result.extra[gift] = count;
    } else if (count < 0) {
      result.missing[gift] = -count;
    }
  });

  return result;
}
  
