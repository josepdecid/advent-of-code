// https://2021.adventjs.dev/challenges/01

export default function contarOvejas(ovejas) {
  return ovejas.filter(({ color, name }) => {
    if (color !== "rojo") {
      return false;
    }

    const n = name.toLowerCase();
    if (!n.includes("a") || !n.includes("n")) {
      return false;
    }

    return true;
  });
}
