// https://adventjs.dev/en/challenges/2024/4

/**
 * @param {number} height - Height of the tree
 * @param {string} ornament - Symbol to draw
 * @returns {string} Drawn tree
 */
function createXmasTree(height, ornament) {
  const ornaments = [...Array.from({ length: height })]
    .map((_, i) => {
      const padding = '_'.repeat(height - (i + 1));
      return padding + ornament.repeat(2 * i + 1) + padding;
    })
    .join('\n');

  const logPadding = '_'.repeat(height - 1);
  const log = logPadding + '#' + logPadding;
  
  return ornaments + '\n' + log + '\n' + log;
}
