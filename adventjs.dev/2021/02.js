// https://2021.adventjs.dev/challenges/02

export default function listGifts(letter) {
  return letter
    .split(" ")
    .reduce((acc, item) => {
      if (item !== "" && !item.startsWith("_")) {
        acc[item] ??= 0;
        ++acc[item];
      }

      return acc;
    }, {});
}
