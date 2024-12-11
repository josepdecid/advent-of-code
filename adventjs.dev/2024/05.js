// https://adventjs.dev/en/challenges/2024/5 

/**
 * @param {{ type: 'I' | 'R', size: number }[]} shoes
 * @returns {number[]} Available shoes 
 */
function organizeShoes(shoes) {
  const result = [];
  const lookup = new Map();

  for (const { type, size } of shoes) {
    const counts = lookup.get(size) || 0;
    const offset = type === 'I' ? 1 : -1;

    if (offset * counts < 0) {
      result.push(size);
    }

    lookup.set(size, counts + offset);
  }

  return result;
}
