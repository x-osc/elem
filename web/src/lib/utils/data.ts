import combinationsJson from "$lib/data/combinations.json";

const combinations: { [comb: string]: string } = combinationsJson;

export function getResult(elem1: string, elem2: string): string | null {
  const comb1 = combinations[elem1 + "|" + elem2];
  const comb2 = combinations[elem2 + "|" + elem1];

  if (comb1) {
    return comb1;
  } else if (comb2) {
    return comb2;
  } else {
    return null;
  }
}
