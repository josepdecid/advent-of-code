// https://2022.adventjs.dev/challenges/2022/11

function getCompleted(part, total) {
  const getSeconds = (timeStr) => {
    const [hours, minutes, seconds] = timeStr.split(':').map(Number);
    return hours * 3600 + minutes * 60 + seconds;
  };

  const partSeconds = getSeconds(part);
  const totalSeconds = getSeconds(total);

  const gcd = (a, b) => {
    while (b) {
      [a, b] = [b, a % b];
    }
    return a;
  };

  const divisor = gcd(partSeconds, totalSeconds);
  const numerator = partSeconds / divisor;
  const denominator = totalSeconds / divisor;

  return `${numerator}/${denominator}`;
}
