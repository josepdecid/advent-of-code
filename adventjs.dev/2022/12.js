// https://2022.adventjs.dev/challenges/2022/12

function selectSleigh(distance, sleighs) {
  let bestSleigh = null;
  for (const sleigh of sleighs) {
    if (sleigh.consumption * distance > 20) break;
    bestSleigh = sleigh.name;
  }

  return bestSleigh;
}
