// https://adventjs.dev/en/challenges/2024/8

/**
 * @param {number[]} indices - The reno indices
 * @param {number} length - The length of the race
 * @returns {string} The reno race
 */
function drawRace(indices, length) {
  return indices.map((val, idx) => {
    const padding = " ".repeat(indices.length - idx - 1);
    const number = " /" + (idx + 1);

    const path = Array.from({ length }).fill("~");
    if (val !== 0) {
      const horsePos = val > 0 ? val : length + val;
      path[horsePos] = "r";
    }

    return padding + path.join("") + number;
  }).join("\n");
}
