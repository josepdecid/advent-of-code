// https://2022.adventjs.dev/challenges/2022/5

function getMaxGifts(giftsCities, maxGifts, maxCities) {
  function* backtrack(start, sum, countCities) {
    if (countCities > maxCities || sum > maxGifts) {
      return;
    }

    if (countCities > 0) {
      yield sum;
    }

    for (let i = start; i < giftsCities.length; i++) {
      yield* backtrack(
        i + 1,
        sum + giftsCities[i],
        countCities + 1
      );
    }
  };

  let maxValidGifts = 0;
  for (const sum of backtrack(0, 0, 0)) {
    if (sum === maxGifts) {
      return maxGifts;
    }

    maxValidGifts = Math.max(maxValidGifts, sum); 
  }

  return maxValidGifts;
}
