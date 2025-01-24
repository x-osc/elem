import configuration.main as config
import data
import serialize


def main():
    config.config()

    print(data.categories)
    print(data.elements)
    print(data.combinations)

    serialize.serialize("../")


if __name__ == "__main__":
    main()
