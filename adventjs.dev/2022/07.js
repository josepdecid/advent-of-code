// https://2022.adventjs.dev/challenges/2022/7

function getGiftsToRefill(a1, a2, a3) {
  const s1 = new Set(a1);
  const s2 = new Set(a2);
  const s3 = new Set(a3);

  const giftsToReplenish = [];
  const allGifts = new Set([...a1, ...a2, ...a3]);
  
  allGifts.forEach((gift) => {
    const stockCount = s1.has(gift) + s2.has(gift) + s3.has(gift);
    if (stockCount === 1) {
      giftsToReplenish.push(gift);
    }
  });

  return giftsToReplenish;
}
