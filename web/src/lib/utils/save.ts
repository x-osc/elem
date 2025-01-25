import { browser } from "$app/environment";
import { elementExists, getElementData } from "./data";

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

function remove_duplicates(array: Array<string>) {
  const seen: { [item: string]: boolean } = {};
  return array.filter(function (item) {
    return Object.hasOwn(seen, item) ? false : (seen[item] = true);
  });
}

export function loadGameSlow(): ElementState {
  const save = loadGameFast();

  // TODO: rewrite to just create a new save dict

  // eslint-disable-next-line prefer-const
  for (let [category, categoryList] of Object.entries(save)) {
    categoryList = categoryList.filter((elem) => elementExists(elem));
    categoryList = remove_duplicates(categoryList);

    console.log(categoryList);
    categoryList.forEach((elem) => {
      if (getElementData(elem)?.category !== category) {
        // delete elem
        const index = categoryList.indexOf(elem);
        if (index > -1) {
          categoryList.splice(index, 1);
        }

        save[category].push(elem);
      }
    });

    save[category] = categoryList;

    if (save[category].length === 0) {
      delete save[category];
    }
  }

  return save;
}
