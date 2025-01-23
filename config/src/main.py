import utils

categories = {}
elements = {}
combinations = {}


def addCategory(id: str, name: str, color="#000000", **kwargs):
    categories[utils.verify_id(id)] = {
        "name": name,
        "color": utils.verify_hex_color(color),
        **kwargs,
    }


def main():
    addCategory("a", "a", color="#d1fd1f")
    print(categories)


if __name__ == "__main__":
    main()
