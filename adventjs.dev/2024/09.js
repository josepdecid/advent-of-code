// https://adventjs.dev/en/challenges/2024/9

/**
 * @param {string[]} board - Represent the train situation
 * @param {'U' | 'D' | 'R' | 'L' } mov - Movement direction
 * @returns {'none' | 'crash' | 'eat'}
 */
function moveTrain(board, mov) {
  const headPosY = board.findIndex(row => row.includes("@"));
  const headPosX = board[headPosY].indexOf("@");

  const offset = {
    'U': [-1, 0],
    'D': [1, 0],
    'L': [0, -1],
    'R': [0, 1]
  }[mov];

  const [i, j] = [headPosY + offset[0], headPosX + offset[1]];
  const value = board[i]?.charAt(j);

  if (value === undefined || value === 'o') {
    return 'crash';
  } 

  if (value === '*') {
    return 'eat';
  }

  return 'none';
}
