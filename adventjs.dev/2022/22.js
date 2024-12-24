// https://2022.adventjs.dev/challenges/2022/22

function checkStepNumbers(systemNames, stepNumbers) {
  const systemSteps = {};

  systemNames.forEach((name, idx) => {
    const step = stepNumbers[idx];
    if (systemSteps[name] === undefined) {
      systemSteps[name] = [step];
    } else {
      systemSteps[name].push(step);
    }
  });

  return Object
    .values(systemSteps)
    .every((steps) => steps
      .slice(0, steps.length - 1)
      .every((step, idx) => step < steps[idx + 1])
    );
}
