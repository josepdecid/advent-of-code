// https://2022.adventjs.dev/challenges/2022/16

function fixLetter(letter) {
  return letter
    .trim()
    .replace(/([,.?!])([^,.?!])/g, '$1 $2')
    .replace(/\s+/g, ' ')
    .replace(/santa claus/gi, 'Santa Claus')
    .replace(/([,.?!]{2,})/g, $1 => $1[0])
    .replace(/\s([,.?!])/g, '$1')
    .replace(/^([A-z])/g, $1 => $1.toUpperCase())
    .replace(/([^.?!])$/g, '$1.')
    .replace(/([.?!])(\s)([A-z])/g,
      (_, $1, $2, $3) => $1 + $2 + $3.toUpperCase()
    );
}
