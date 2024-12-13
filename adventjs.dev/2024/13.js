// https://adventjs.dev/en/challenges/2024/13

/** @param {string[]} moves
 * @returns {true|[number, number]} Return true if robot returns or position
 */
function isRobotBack(moves) {
  const moveToOffset = {L: [-1, 0], R: [1, 0], U: [0, 1], D: [0, -1]};
  const moveToOpposite = {L: "R", R: "L", U: "D", D: "U"};

  const position = [0, 0];
  const previousMoves = new Set();

  for (let i = 0; i < moves.length; ++i) {
    let move = moves[i];
    let offset = moveToOffset[move];

    if (!offset) {
      const modifier = moves[i++];
      move = modifier === "!" ? moveToOpposite[moves[i]] : moves[i];

      if (modifier === "?" && previousMoves.has(move)) {
        continue;
      }

      const multiplier = modifier === "*" ? 2 : 1;
      offset = [
        moveToOffset[move][0] * multiplier,
        moveToOffset[move][1] * multiplier,
      ];
    }

    position[0] += offset[0];
    position[1] += offset[1];
    previousMoves.add(move);
  }

  return position[0] === 0 && position[1] === 0 ? true : position;
}
