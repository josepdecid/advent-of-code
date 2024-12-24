// https://2022.adventjs.dev/challenges/2022/10

function checkJump(heights) {
  let hasAscended = false;
  let hasDescended = false;
  let ascending = true;

  for (let i = 0; i < heights.length - 1; ++i) {
    if (heights[i] === heights[i + 1]) {
      continue;
    }

    const isAscending = heights[i] < heights[i + 1];
    if (isAscending) {
      hasAscended = true;
    } else {
      hasDescended = true;
    }

    if (!ascending && isAscending) {
      return false;
    }

    if (ascending && !isAscending) {
      ascending = false;
    }
  }

  return hasAscended && hasDescended;
}
