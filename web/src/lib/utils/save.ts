import { browser } from "$app/environment";
import { elementExists } from "./data";

export type ElementState = { [key: string]: string[] };
const key = "gamesave";

export function saveGame(state: ElementState) {
  if (!browser) {
    return;
  }

  localStorage.setItem(key, JSON.stringify(state));
}

export function loadGameFast(): ElementState {
  if (!browser) {
    return {};
  }

  const gamesave = localStorage.getItem(key);
  if (gamesave) {
    return JSON.parse(gamesave);
  }

  return {
    air: ["air"],
    earth: ["earth"],
    fire: ["fire"],
    water: ["water"],
  };
}

export function loadGameSlow(): ElementState {
  const save = loadGameFast();

  for (const [category, categoryList] of Object.entries(save)) {
    save[category] = categoryList.filter((elem) => elementExists(elem));
    if (save[category].length === 0) {
      delete save[category];
    }
  }

  return save;
}
