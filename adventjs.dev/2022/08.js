// https://2022.adventjs.dev/challenges/2022/8

function checkPart(part) {
  const substrings = [...Array(part.length)]
    .map((_, i) => part.slice(0, i) + part.slice(i + 1));

  return [part, ...substrings]
    .some((str) => str === str.split("").reverse().join(""));
}
