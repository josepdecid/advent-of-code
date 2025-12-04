function manufactureGifts(
  giftsToProduce: Array<{toy: string; quantity: number}>
): string[] {
  return giftsToProduce
    .filter(({quantity}) => quantity > 0)
    .reduce<string[]>(
      (acc, {toy, quantity}) => acc.concat(Array(quantity).fill(toy)),
      []
    );
}
