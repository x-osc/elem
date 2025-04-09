import data
import utils

aliases: dict[str, list[str]] = {}


def category(id: str, name: str, color=None, **kwargs):
    data.add_category(id, name, color=color, **kwargs)


def element(id: str, name: str, category_id: str, color=None, **kwargs):
    if color is None:
        color = data.categories[category_id].get("color")

    data.add_element(id, name, category_id, color=color, **kwargs)


def combination(result: str, elem1: str, elem2: str):
    if elem1.startswith("$"):
        alias_elems = aliases.get(elem1[1:])
        if alias_elems is None:
            return ValueError("Alias is not vaild")

        for elem in alias_elems:
            combination(result, elem, elem2)

        return
    

    if elem2.startswith("$"):
        alias_elems = aliases.get(elem2[1:])
        if alias_elems is None:
            return ValueError("Alias is not vaild")

        for elem in alias_elems:
            combination(result, elem1, elem)

        return

    data.add_combination(result, elem1, elem2)


def alias(id: str, elems: list[str]):
    if not id.startswith("$"):
        raise ValueError("Alias must start with $")

    if not all(elem in data.elements.keys() for elem in elems):
        raise ValueError("All aliases must be vaild element ids, some do not exist")

    aliases[utils.verify_id(id[1:])] = elems
