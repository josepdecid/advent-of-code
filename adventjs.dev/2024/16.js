// https://adventjs.dev/challenges/2024/16

/**
 * @param {string} s
 * @returns {string}
 */
function removeSnow(s) {
  const stack = [];

  for (const val of s) {
    stack.length > 0 && stack[stack.length - 1] === val
      ? stack.pop()
      : stack.push(val);
  };
  
  return stack.join("");
}

