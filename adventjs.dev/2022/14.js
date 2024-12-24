// https://2022.adventjs.dev/challenges/2022/14

function getOptimalPath(path) {
  const memo = {};
  function getMemoPath(level, idx) {
    if (memo[level] === undefined) {
      memo[level] = {};
    }

    if (memo[level][idx] === undefined) {
      memo[level][idx] = computePaths(level, idx);
    }

    return memo[level][idx];
  }

  function computePaths(level, idx) {
    if (level === path.length - 1) {
      return path[level][idx];
    }

    return path[level][idx] + Math.min(
      getMemoPath(level + 1, idx),
      getMemoPath(level + 1, idx + 1)
    );
  }

  const bestPath = computePaths(0, 0, 0);
  return bestPath;
}
