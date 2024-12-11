// https://adventjs.dev/en/challenges/2024/1

function prepareGifts(gifts) {
  return [...new Set(gifts)].sort((a, b) => a - b);
}

