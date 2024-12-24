// https://2021.adventjs.dev/challenges/06

export default function sumPairs(numbers, result) {
  for (let i = 0; i < numbers.length - 1; ++i) {
    const x = numbers[i];
    for (let j = i + 1; j < numbers.length; ++j) {
      const y = numbers[j];
      if (x + y === result) {
        return [x, y];
      }
    }
  }

  return null;
}
