// https://adventjs.dev/challenges/2024/24

/**
 * @param {object} tree1 - The first binary tree.
 * @param {object} tree2 - The second binary tree.
 * @returns {[boolean, string]}
 */
function isTreesSynchronized(tree1, tree2) {
  if (!tree1 && !tree2) return [true, ""];
  if (!tree1 || !tree2) return [false, tree1?.value || ""];
  
  const synchronized =
    tree1.value === tree2.value &&
    isTreesSynchronized(tree1.left, tree2.right)[0] && 
    isTreesSynchronized(tree1.right, tree2.left)[0];
  
  return [synchronized, tree1.value];
}

