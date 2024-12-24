// https://2022.adventjs.dev/challenges/2022/1

function wrapping(gifts) {
  return gifts.map((gift) => {
    const borders = "*".repeat(gift.length + 2);
    return borders + "\n*" + gift + "*\n" + borders;
  });
}

