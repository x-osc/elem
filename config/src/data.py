import utils

categories: dict[str, dict] = {}
elements: dict[str, dict] = {}
combinations: dict[tuple[str, str], str] = {}


def add_category(id: str, name: str, color=None, **kwargs):
    categories[utils.verify_id(id)] = {
        "name": name,
        "color": None if color is None else utils.verify_hex_color(color),
        **kwargs,
    }


def add_element(id: str, name: str, category_id: str, color=None, **kwargs):
    if category_id not in categories.keys():
        raise ValueError(
            "Category id of element does not exist (is the category defined before this element?)"
        )

    if color is None and categories[id].get("color") is None:
        raise Exception(
            "Neither element or category defines a color, must specify color"
        )

    elements[utils.verify_id(id)] = {
        "name": name,
        "category": category_id,
        "color": None if color is None else utils.verify_hex_color(color),
        **kwargs,
    }


def add_combination(result: str, elem1: str, elem2: str):
    if result not in elements.keys():
        raise Exception("Result element id does not exist")
    if any(id not in elements.keys() for id in (elem1, elem2)):
        raise Exception("Combination element id does not exist")

    combinations[(elem1, elem2)] = result


def get_combination(elem1: str, elem2: str) -> str | None:
    comb1 = combinations.get((elem1, elem2))
    if comb1 is not None:
        return comb1

    comb2 = combinations.get((elem2, elem1))
    if comb2 is not None:
        return comb2

    return None
