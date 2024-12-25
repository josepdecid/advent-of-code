// https://2023.adventjs.dev/challenges/2023/1

function findFirstRepeated(gifts) {
  const visited = new Set();
  for (const gift of gifts) {
    if (visited.has(gift)) {
      return gift;
    }
    
    visited.add(gift);
  }

  return -1;
}
