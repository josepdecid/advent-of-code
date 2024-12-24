// https://2021.adventjs.dev/challenges/05

export default function daysToXmas(date) {
  const xmas = new Date('Dec 25, 2021');
  const timeDiff = xmas.getTime() - date.getTime();
  return Math.ceil(timeDiff / (1000 * 3600 * 24));
}
