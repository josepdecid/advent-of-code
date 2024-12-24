// https://2022.adventjs.dev/challenges/2022/6

function createCube(size) {
  let result = [];

  for (let i = 0; i < size; ++i) {
    const space = " ".repeat(size - i - 1);
    const middle = "/\\".repeat(i + 1);
    const side = "_\\".repeat(size);
    result.push(space + middle + side);
  }

  for (let i = 0; i < size; ++i) {
    const space = " ".repeat(i);
    const middle = "\\/".repeat(size - i);
    const side = "_/".repeat(size);
    result.push(space + middle + side);
  }

  return result.join("\n");
}
