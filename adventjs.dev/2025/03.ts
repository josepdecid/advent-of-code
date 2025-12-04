function drawGift(size: number, symbol: string): string {
  if (size < 2) return "";

  const borderLine = Array(size).fill(symbol).join("");
  const centerLine = symbol + Array(size - 2).fill(" ").join("") + symbol;

  return [
    borderLine,
    ...Array(size - 2).fill(centerLine),
    borderLine
  ].join("\n");
}