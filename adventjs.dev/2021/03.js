// https://2021.adventjs.dev/challenges/03

export default function isValid(letter) {
  let count = 0;
  
  for (let i = 0; i < letter.length; ++i) {
    const c = letter[i];

    if (c === "(") {
      if (letter[i + 1] === ")") {
        return false;
      } 
      ++count;
    }

    else if (c === ")") {
      if (count === 0) {
        return false;
      }
      --count;
    }

    else if (['{', '['].includes(c)) {
      return false;
    }
  }

  return count === 0;
}
