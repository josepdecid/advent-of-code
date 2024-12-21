// https://adventjs.dev/challenges/2024/19

/**
 * @param {number} weight - Total weight of the gifts
 * @returns {string} - Stacked boxes represented as ASCII art
 */
function distributeWeight(weight) {
  const boxes = [10, 5, 2, 1];
  const boxRepresentations = {
    1: [" _ ", "|_|"],
    2: [" ___ ", "|___|"],
    5: [" _____ ", "|     |", "|_____|"],
    10: [" _________ ", "|         |", "|_________|"]
  };

  const usedBoxes = [];
  let remainingWeight = weight;
  
  boxes.forEach((box) => {
    while (remainingWeight >= box) {
      usedBoxes.push(box);
      remainingWeight -= box;
    }
  });

  usedBoxes.reverse();
  
  const result = [];
  let currentLine = 0;
  
  usedBoxes.forEach((box) => {
    const boxLines = boxRepresentations[box];
    
    boxLines.forEach((line, lineIdx) => {      
      if (lineIdx > 0 || currentLine === 0) {
        currentLine++;
        result.push(line);
        return;
      }

      const lastLine = result[currentLine - 1];
      
      const overlappedLine = lastLine
        .split("")
        .map((char, charIdx) => char === "|" ? "|" : line[charIdx])
        .join("");

      const newLine = line.substring(lastLine.length).trimRight();

      result[currentLine - 1] = overlappedLine + newLine;
    });
  });
  
  return result.join("\n");
}
      
