// https://adventjs.dev/en/challenges/2024/10

/**
 * @param {string[]} instructions - The instructions to execute
 * @returns {number} The value of the register A
 */
function compile(instructions) {
  let memory = new Map();
  const getSafeValue = (register) => memory.get(register) || 0;
  
  let i = 0;
  while (i < instructions.length) {
    const parts = instructions[i].split(' ');
    
    if (parts[0] === 'JMP') {
      if (getSafeValue(parts[1]) === 0) {
        i = parseInt(parts[2]);
        continue;
      }
    }
    
    else if (parts[0] === 'MOV') {
      const val = isNaN(parts[1]) ? getSafeValue(parts[1]) : parseInt(parts[1]);
      memory.set(parts[2], val);
    }

    else {
      const offset = parts[0] === 'INC' ? 1 : -1;
      memory.set(parts[1], getSafeValue(parts[1]) + offset);
    }

    ++i;
  }

  return memory.get('A');
}
