import data


def category(id: str, name: str, color=None, **kwargs):
    data.add_category(id, name, color=color, **kwargs)


def element(id: str, name: str, category_id: str, color=None, **kwargs):
    if color is None:
        color = data.categories[category_id].get("color")

    data.add_element(id, name, category_id, color=color, **kwargs)


def combination(result: str, elem1: str, elem2: str):
    data.add_combination(result, elem1, elem2)
