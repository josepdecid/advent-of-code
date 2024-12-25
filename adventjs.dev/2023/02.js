// https://2023.adventjs.dev/en/challenges/2023/2

function manufacture(gifts, materials) {
  return gifts.filter((gift) =>
    gift.split("").every((part) => materials.includes(part))
  );
}
