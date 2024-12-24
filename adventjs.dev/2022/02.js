// https://2022.adventjs.dev/en/challenges/2022/2

function countHours(year, holidays) {
  return holidays.reduce((count, holiday) => {
    const date = new Date(`${year}/${holiday}`);
    if (date.getDay() > 0 && date.getDay() < 6) {
      count += 2;
    }

    return count;
  }, 0);
}

