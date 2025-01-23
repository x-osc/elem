import data
from config_helpers import category, combination, element


def main():
    category("a", "a", color="#d1fd1f")
    element("asdf", "Asdf", "a")
    combination("asdf", "asdf", "asdf")

    print(data.categories)
    print(data.elements)
    print(data.combinations)


if __name__ == "__main__":
    main()
