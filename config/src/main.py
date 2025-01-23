import data


def main():
    data.add_category("a", "a", color="#d1fd1f")
    data.add_element(
        "asdf",
        "Asdf",
        "a",
    )
    data.add_combination("asdf", "asdf", "asdf")

    print(data.categories)
    print(data.elements)
    print(data.combinations)


if __name__ == "__main__":
    main()
