import categoryJson from "$lib/data/categories.json";
import combinationsJson from "$lib/data/combinations.json";
import elementsJson from "$lib/data/elements.json";

export type CategoryData = {
  name: string;
};

export type ElementData = {
  name: string;
  category: string;
  tier: number;
  color: string;
};

const categories: { [id: string]: CategoryData } = categoryJson;
const combinations: { [comb: string]: string } = combinationsJson;
const elements: { [id: string]: ElementData } = elementsJson;

export function getResult(elem1: string, elem2: string): string | null {
  const comb1 = combinations[elem1 + "|" + elem2];
  if (comb1) {
    return comb1;
  }

  const comb2 = combinations[elem2 + "|" + elem1];
  if (comb2) {
    return comb2;
  }

  return null;
}

export function getElementData(id: string): ElementData | null {
  return elements[id] ?? null;
}

export function elementExists(id: string) {
  return Object.hasOwn(elements, id);
}

export function getCategoryData(id: string): CategoryData | null {
  return categories[id] ?? null;
}
