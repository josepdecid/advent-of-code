// https://adventjs.dev/challenges/2024/15

/**
  * @param {Array<Object>} data
  * @returns {string}
  */
function drawTable(data) {
  const columns = Object.keys(data[0]);
  
  const widths = {};
  for (const column of columns) {
    widths[column] = Math.max(
      column.length,
      ...data.map((row) => String(row[column]).length)
    );
  }

  const separator = "+" + columns.map((col) => "-".repeat(widths[col] + 2)).join("+") + "+";

  const header = `| ${columns.map((col) => 
    col.charAt(0).toUpperCase() + col.slice(1).padEnd(widths[col] - 1)
  ).join(" | ")} |`;
  
  const rows = data.map((row) =>
    `| ${columns.map((col) => String(row[col]).padEnd(widths[col])
  ).join(" | ")} |`);
  
  return [
    separator,
    header,
    separator,
    ...rows,
    separator
  ].join("\n");
}

