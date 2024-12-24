// https://2022.adventjs.dev/challenges/2022/24

function canExit(maze) {
  let i, j;
  for (i = 0; i < maze.length; ++i) {
    j = maze[i].findIndex((val) => val === "S");
    if (j !== -1) break;
  }

  const queue = [[i, j]];
  while (queue.length > 0) {
    [i, j] = queue.shift();

    const value = maze[i] === undefined
      ? undefined
      : maze[i][j];

    if (value === "W" || value === undefined) {
      continue;
    }

    if (value === "E") {
      return true;
    }

    maze[i][j] = "W";
    
    queue.push([i - 1, j]);
    queue.push([i + 1, j]);
    queue.push([i, j - 1]);
    queue.push([i, j + 1]);
  }

  return false;
}
