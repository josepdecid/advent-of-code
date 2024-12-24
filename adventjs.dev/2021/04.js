// https://2021.adventjs.dev/challenges/04

export default function createXmasTree(height) {
  const tree = [];
  
  for (let i = 0; i < height; ++i) {
    const side = "_".repeat(height - i - 1);
    const leaves = "*".repeat(2 * i + 1);
    tree.push(side + leaves + side);
  }

  const logSide = "_".repeat(height - 1);
  tree.push(logSide + "#" + logSide);
  tree.push(logSide + "#" + logSide);

  return tree.join("\n");
}
