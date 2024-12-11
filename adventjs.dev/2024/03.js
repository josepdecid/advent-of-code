// https://adventjs.dev/en/challenges/2024/3

/**
 * @param {{ name: string, quantity: number, category: string }[]} inventory
 * @returns {object} The organized inventory
 */
function organizeInventory(inventory) {
  return inventory.reduce((groups, { name, quantity, category }) => {
    groups[category] = groups[category] ?? {};
    groups[category][name] = (groups[category][name] ?? 0) + quantity;
    return groups;
  }, {});
}
