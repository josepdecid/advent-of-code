// https://adventjs.dev/challenges/2024/25

/**
 * @param {string} code - The magical program to execute
 * @returns {number} - The final value after executing the program
 */
function execute(code) {
  let result = 0;
  let loopIdx = -1;

  let i = 0;
  while (i < code.length) {
    const op = code[i];

    if (op === "{" && result === 0) {
      while (code[i] !== "}") ++i;
    }

    else if (op === "[") loopIdx = i;
    else if (op === "]" && result !== 0) i = loopIdx;

    else if (op === "+") ++result;
    else if (op === "-") --result;

    ++i;
  }

  return result;
}

