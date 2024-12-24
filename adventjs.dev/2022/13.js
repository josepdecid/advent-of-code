// https://2022.adventjs.dev/challenges/2022/13

function getFilesToBackup(lastBackup, changes) {
  const filesWithChanges = new Set(changes
    .filter(([, timestamp]) => timestamp > lastBackup)
    .map(([id]) => id)
  );

  return [...filesWithChanges].sort((a, b) => a - b);
}
