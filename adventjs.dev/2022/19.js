// https://2022.adventjs.dev/challenges/2022/19

function sortToys(toys, positions) {
  return toys
    .map((toy, idx) => [toy, positions[idx]])
    .sort(([, a], [, b]) => a - b)
    .map(([toy]) => toy);
}
