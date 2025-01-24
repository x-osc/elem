from pathlib import Path

import configuration.main as config
import data
import serialize


def main():
    config.config()

    print(data.categories)
    print(data.elements)
    print(data.combinations)

    serialize.serialize(Path(__file__).parents[2])


if __name__ == "__main__":
    main()
