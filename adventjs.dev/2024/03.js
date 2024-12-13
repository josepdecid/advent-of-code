/**
 * @param {{ name: string, quantity: number, category: string }[]} inventory
 * @returns {object} The organized inventory
 */
function organizeInventory(inventory) {
  const result = {};

  for (const { name, quantity, category } of inventory) {
    result[category] ??= {};
    result[category][name] ??= 0;
    result[category][name] += quantity;
  }

  return result;
}