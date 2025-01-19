import combinationsJson from "$lib/data/combinations.json";
import elementsJson from "$lib/data/elements.json";

export type ElementData = {
  name: string;
  category: string;
  tier: number;
  color: string;
};

const combinations: { [comb: string]: string } = combinationsJson;
const elements: { [id: string]: ElementData } = elementsJson;

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

export function getElementData(id: string): ElementData | null {
  return elements[id] ?? null;
}

export function elementExists(id: string) {
  return Object.hasOwn(elements, id);
}
