// https://adventjs.dev/en/challenges/2024/7

/** @param {string} packages with parentheses
 *  @returns {string} Fixed and sorted packages
 */
function fixPackages(packages) {
  const stack = [];
  const pairs = [];

  let result = packages.split('');
  result.forEach((val, idx) => {
    if (val === '(') {
      stack.push(idx);
    } 
    
    else if (val === ')' && stack.length > 0) {
      pairs.push([stack.pop(), idx]);
    }
  });

  pairs.sort((a, b) => b[0] - a[0]);

  for (const [start, end] of pairs) {
    const content = result.slice(start + 1, end);
    const reversed = content.reverse();
    result.splice(start + 1, end - start - 1, ...reversed);
  }
    
  return result.filter((val) => val !== '(' && val !== ')').join('');
}

