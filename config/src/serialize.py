import json
from pathlib import Path

import data


def serialize(path):
    with open(Path(path) / "elements.json", "w") as f:
        elems = data.elements

        json.dump(elems, f)

    with open(Path(path) / "categories.json", "w") as f:
        cats = data.categories

        json.dump(cats, f)
    with open(Path(path) / "combinations.json", "w") as f:
        combs = data.combinations

        new_combs = {}
        for comb, res in combs.items():
            new_combs[comb[0] + "|" + comb[1]] = res

        json.dump(new_combs, f)
