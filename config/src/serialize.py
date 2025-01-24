import json
import os

import data


def serialize(path):
    with open(os.path.join(path, "elements.json"), "w") as f:
        elems = data.elements

        json.dump(elems, f)

    with open(os.path.join(path, "categories.json"), "w") as f:
        cats = data.categories

        json.dump(cats, f)
    with open(os.path.join(path, "combinations.json"), "w") as f:
        combs = data.combinations

        new_combs = {}
        for comb, res in combs.items():
            new_combs[comb[0] + "|" + comb[1]] = res

        json.dump(new_combs, f)
