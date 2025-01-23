import utils

categories = {}
elements = {}
combinations = {}


def add_category(id: str, name: str, color=None, **kwargs):
    categories[utils.verify_id(id)] = {
        "name": name,
        "color": None if color is None else utils.verify_hex_color(color),
        **kwargs,
    }


def add_element(id: str, name: str, category_id: str, color=None, **kwargs):
    if category_id not in categories.keys():
        raise ValueError("Category does not exist")

    if color is None:
        new_color = categories[category_id].get("color")
        if new_color is None:
            raise Exception("Category has no color, must specify color")
        color = new_color
    else:
        color = utils.verify_hex_color(color)

    elements[utils.verify_id(id)] = {
        "name": name,
        "category": category_id,
        "color": color,
        **kwargs,
    }


def main():
    add_category("a", "a", color="#d1fd1f")
    add_element(
        "asdf",
        "Asdf",
        "a",
    )

    print(categories)
    print(elements)


if __name__ == "__main__":
    main()
