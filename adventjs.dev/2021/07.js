// https://2021.adventjs.dev/challenges/07

export default function contains(store, product) {
  if (typeof store === "string") {
    return store === product;
  }

  return Object
    .values(store)
    .some((value) => contains(value, product));
}
