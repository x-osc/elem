import { base } from "$app/paths";

export type CategoryData = {
  name: string;
  amount: number;
};

export type ElementData = {
  name: string;
  category: string;
  tier: number;
  color: string;
  used_in: number;
};

let categories: { [id: string]: CategoryData };
let combinations: { [comb: string]: string };
let elements: { [id: string]: ElementData };

export async function loadData() {
  categories = await fetch(`${base}/data/categories.json`).then((res) =>
    res.json()
  );
  combinations = await fetch(`${base}/data/combinations.json`).then((res) =>
    res.json()
  );
  elements = await fetch(`${base}/data/elements.json`).then((res) =>
    res.json()
  );
}

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
