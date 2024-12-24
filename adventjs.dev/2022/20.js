// https://2022.adventjs.dev/challenges/2022/20

function howManyReindeers(reindeerTypes, gifts) {
  return gifts.map((gift) => {
    let max = gift.weight;
    const reindeers = reindeerTypes
      .map((x) => [x.type, x.weightCapacity])
      .filter(([, x]) => x < max)
      .sort(([, a], [, b]) => a - b);

    const res = reindeers.map(([type]) => ({ type, num: 0 }));
    reindeers.forEach((_, i) => {
      const sliced = reindeers.slice(0, reindeers.length - i)
      const sum = sliced.reduce((sum, e) => sum + e[1], 0);
      sliced.forEach((_, i) => res[i].num += Math.floor(max / sum));
      max %= sum;
    });

    return {
      country: gift.country,
      reindeers: res.reverse(),
    };
  });
}
