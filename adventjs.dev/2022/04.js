// https://2022.adventjs.dev/challenges/2022/4

function fitsInOneBox(boxes) {
  const sortedBoxes = boxes.sort((a, b) => 
    (a.l * a.w * a.h) - (b.l * b.w * b.h)
  )

  for (let i = 0; i < sortedBoxes.length - 1; i++) {
    const current = sortedBoxes[i];
    const next = sortedBoxes[i + 1];
    
    if (
      current.l >= next.l || 
      current.w >= next.w || 
      current.h >= next.h
    ) {
      return false;
    }
  }
  
  return true;
}
