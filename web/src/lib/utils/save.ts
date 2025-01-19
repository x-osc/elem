import { browser } from "$app/environment";

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
