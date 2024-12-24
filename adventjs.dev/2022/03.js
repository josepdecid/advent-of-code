// https://2022.adventjs.dev/en/challenges/2022/3

function distributeGifts(packOfGifts, reindeers) {
  return Math.floor(
    reindeers.reduce((s, x) => s + 2 * x.length, 0) /
    packOfGifts.reduce((s, x) => s + x.length, 0)
  );
}
