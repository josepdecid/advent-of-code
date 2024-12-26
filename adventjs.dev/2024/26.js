// https://adventjs.dev/es/challenges/2024/26

/**
 * @param {string} timeWorked - Time worked in hh:mm:ss format.
 * @param {string} totalTime - Total estimated time in hh:mm:ss format.
 * @returns {string} - The completed percentage rounded to the nearest integer with a % sign.
 */
function getCompleted(timeWorked, totalTime) {
  function getSeconds(time) {
    const [h, m, s] = time.split(":").map((x) => parseInt(x, 10));
    return (60 * 60 * h) + (60 * m) + s;
  }
  
  return `${Math.round(
    100 * (getSeconds(timeWorked) / getSeconds(totalTime))
  )}%`;
}
 
