// https://2022.adventjs.dev/challenges/2022/23

function executeCommands(commands) {
  const registers = Array.from({ length: 8 }).fill(0);
  const getRegisterIndex = (register) => parseInt(register[2], 10);
  const overflow = (n) => ((n % 256) + 256) % 256;

  let i = 0;
  while (i < commands.length) {
    const [instruction, args] = commands[i].split(" ");

    if (instruction === "JMP") {
      if (overflow(registers[0]) !== 0) {
        i = parseInt(args, 10) - 1;
      }
    }
    else if (instruction === "INC") {
      ++registers[getRegisterIndex(args)];
    }
    else if (instruction === "DEC") {
      --registers[getRegisterIndex(args)];
    }
    else if (instruction === "ADD") {
      const [reg1, reg2] = args.split(",");
      registers[getRegisterIndex(reg1)] += registers[getRegisterIndex(reg2)];
    }
    else {
      const [source, target] = args.split(",");
      const value = parseInt(source, 10);
      registers[getRegisterIndex(target)] = isNaN(value)
        ? registers[getRegisterIndex(source)]
        : value;
    }

    ++i;
  }

  return registers.map((value) => overflow(value));
}
