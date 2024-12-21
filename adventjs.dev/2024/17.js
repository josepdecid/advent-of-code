// https://adventjs.dev/challenges/2024/17

/**
 * @param {boolean[][]} grid
 * @returns {number[][]}
 */
function detectBombs(grid) {
  const rows = grid.length;
  const cols = grid[0].length;

  const counts = [];
  const offsets = [
    [-1, -1], [-1, 0], [-1, 1], [0, -1],
    [0, 1], [1, -1], [1, 0], [1, 1]
  ];

  for (let i = 0; i < rows; ++i) {
    const rowCounts = [];
    counts.push(rowCounts);

    for (let j = 0; j < cols; ++j) {
      const adjacentCounts = offsets
        .filter(([oi, oj]) => grid[i + oi]?.[j + oj])
        .length
      
      rowCounts.push(adjacentCounts);
    }
  }

  return counts;
}
 
