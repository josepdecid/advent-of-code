// https://2022.adventjs.dev/challenges/2022/17

function carryGifts(gifts, maxWeight) {
  const result = [];
  
  let currentGroup = [];
  let currentWeight = 0;

  gifts
    .filter((gift) => gift.length <= maxWeight)
    .forEach((gift) => {
      if (currentWeight + gift.length > maxWeight) {
        result.push(currentGroup.join(" "));
        currentGroup = [];
        currentWeight = 0;
      }

      currentGroup.push(gift);
      currentWeight += gift.length;
    });

  if (currentGroup.length > 0) {
    result.push(currentGroup.join(" "));
  }

  return result;
}
