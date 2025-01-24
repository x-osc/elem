from config_helpers import alias, category, combination, element


def config():
    category("air", "Air", color="#97dafb")
    category("earth", "Earth", color="#5177f3")
    category("fire", "Fire", color="#f57f43")
    category("water", "Water", color="#734f31")

    element("air", "Air", "air")
    element("earth", "Earth", "earth")
    element("fire", "Fire", "fire")
    element("water", "Water", "water")

    element("steam", "Steam", "air")

    element("mud", "Mud", "earth")
    element("igneous_rock", "Igneous Rock", "earth")

    element("lava", "Lava", "fire")

    alias("$base", ["air", "earth", "fire", "water"])

    combination("steam", "fire", "water")
    combination("mud", "water", "earth")
    combination("lava", "fire", "earth")
    combination("igneous_rock", "lava", "air")
