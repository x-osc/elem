from data import add_category, add_combination, add_element


def category(id: str, name: str, color=None, **kwargs):
    add_category(id, name, color=color, **kwargs)


def element(id: str, name: str, category_id: str, color=None, **kwargs):
    add_element(id, name, category_id, color=color, **kwargs)


def combination(result: str, elem1: str, elem2: str):
    add_combination(result, elem1, elem2)
