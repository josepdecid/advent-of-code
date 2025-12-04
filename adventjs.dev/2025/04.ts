function decodeSantaPin(code: string): string | null {
  const blocks = [...code.matchAll(/\[([^\]]+)\]/g)];
  if (blocks.length !== 4) {
    return null;
  }

  const values: number[] = [];
  blocks.forEach(([_, block], i) => {
    const rawNumber = block[0];
    let currentNumber =
      rawNumber === "<" ? values[i - 1] : parseInt(rawNumber, 10);

    for (let j = 1; j < block.length; ++j) {
      const offset = block[j] === "+" ? 1 : -1;
      currentNumber += offset;
    }

    values.push(currentNumber);
  });

  const mod10 = (n: number) => ((n % 10) + 10) % 10;
  return values.map(mod10).join("");
}
