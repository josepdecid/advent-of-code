// https://adventjs.dev/challenges/2024/18

/**
 * @param {string} agenda
 * @param {string} phone
 * @returns {{ name: string, address: string } | null}
 */
function findInAgenda(agenda, phone) {
  const phoneRegex = /\+\d{1,2}-\d{3}-\d{3}-\d{3}/g;
  const nameRegex = /<([^>]+)>/g;

  let result = null;
  
  const entries = agenda.split("\n");
  for (const entry of entries) {
    // Number does not match for this entry
    const matchesNumber = entry.match(phoneRegex)[0]?.includes(phone);
    if (!matchesNumber) continue;
    
    // Too many results
    if (result !== null) return null;
    
    const nameMatch = nameRegex.exec(entry);
    const name = nameMatch[1];
    nameRegex.lastIndex = 0;

    result = {
      name,
      address: entry
        .replace(phoneRegex, "")
        .replace(nameRegex, "")
        .trim()
    };
  }
  
  return result;
}

