// https://2022.adventjs.dev/challenges/2022/18

function dryNumber(dry, numbers) {
  const value = dry.toString();
  const result = [];
  
  for (let i = 1; i <= numbers; ++i) {
    if (i.toString().includes(value)) {
      result.push(i);
    }
  }

  return result;
}
