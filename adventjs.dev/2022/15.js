// https://2022.adventjs.dev/challenges/2022/15

function decorateTree(base) {
  const getTopDecoration = (left, right) => left === right
    ? left
    : ["B", "R", "P"].find(v => v !== left && v !== right);

  const decorations = [base.split(" ")];
  let i = decorations[0].length - 1;
  while (i > 0) {
    const previous = decorations[decorations.length - 1];
    decorations.push([]);

    for (let j = 0; j < i; ++j) {
      const decoration = getTopDecoration(previous[j], previous[j + 1]);
      decorations[decorations.length - 1].push(decoration);
    }

    --i;
  }

  return decorations
    .reverse()
    .map((row) => row.join(" "));
}
