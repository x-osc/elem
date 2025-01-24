import configuration.main as config
import data


def main():
    config.config()

    print(data.categories)
    print(data.elements)
    print(data.combinations)


if __name__ == "__main__":
    main()
