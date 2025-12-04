/**
 * @param {Array<{ toy: string, quantity: number }>} giftsToProduce
 * @returns {string[]} Array of manufactured gifts
 */
function manufactureGifts(giftsToProduce) {
  return giftsToProduce
    .filter(({quantity}) => quantity > 0)
    .reduce(
      (acc, {toy, quantity}) => acc.concat(Array(quantity).fill(toy)),
      []
    );
}
