// https://adventjs.dev/en/challenges/2024/7

/** @param {string} packages with parentheses
 *  @returns {string} Fixed and sorted packages
 */
function fixPackages(packages) {
  while (packages.includes("(")) {
        const startIdx = packages.lastIndexOf("(");
        const endIdx = packages.indexOf(")", startIdx);
        
        const group = packages.substring(startIdx, endIdx + 1);
        const reverse = [...group].slice(1, -1).reverse().join("");

        packages = packages.replace(group, reverse);
    } 
  
  return packages;
}
