// https://2022.adventjs.dev/challenges/2022/21

function printTable(gifts) {
  const data = [{ name: "Gift", quantity: "Quantity" }];
  gifts.forEach(({ name, quantity }) => {
    data.push({ name, quantity: String(quantity) });
  });

  const giftWidth = Math.max(
    ...data.map(({ name }) => name.length));
  const quantityWidth = Math.max(
    ...data.map(({ quantity }) => quantity.length));

  const sep1 = "+".repeat(2 + giftWidth + 3 + quantityWidth + 2);
  const sep2 = `| ${"-".repeat(giftWidth)} | ${"-".repeat(quantityWidth)} |`;
  const sep3 = "*".repeat(2 + giftWidth + 3 + quantityWidth + 2);
  const rows = data.map(({ name, quantity }) => "| "
    + name.padEnd(giftWidth) + " | "
    + quantity.padEnd(quantityWidth) + " |");
  
  return [sep1, rows[0], sep2, ...rows.splice(1), sep3].join("\n");
}
