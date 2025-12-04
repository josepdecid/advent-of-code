function filterGifts(gifts: string[]): string[] {
  return gifts.filter((value) => !value.includes("#"));
}
