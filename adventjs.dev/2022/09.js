// https://2022.adventjs.dev/challenges/2022/9

function countTime(leds) {
  let seconds = 0;
  
  while (leds.some((l) => l === 0)) {
    const newLeds = [...leds];
    leds.forEach((l, idx) => {
      const nextIdx = (idx + 1) % leds.length;
      if (l === 1) {
        newLeds[nextIdx] = 1;
      }
    });

    leds = newLeds;
    seconds += 7;
  }

  return seconds;
}
