from config_helpers import alias, category, combination, element


def config():
    category("a", "a", color="#d1fd1f")

    element("a", "a", "a")
    element("b", "b", "a")
    element("c", "c", "a")
    element("d", "d", "a")

    alias("$base", ["a", "b"])

    combination("a", "$base", "a")
