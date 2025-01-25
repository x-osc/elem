from config_helpers import alias, category, combination, element


# fmt: off
def config():
    category("air", "Air", color="#97dafb")
    category("earth", "Earth", color="#734f31")
    category("fire", "Fire", color="#f57f43")
    category("water", "Water", color="#5177f3")

    category("rock", "Rock", color="#555555")

    #

    element("air", "Air", "air")
    element("earth", "Earth", "earth")
    element("fire", "Fire", "fire")
    element("water", "Water", "water")

    element("wind", "Wind", "air")
    element("breeze", "Breeze", "air")
    element("gale", "Gale", "air")

    element("storm", "Storm", "air")
    element("wind_storm", "Wind Storm", "air")

    element("tornado", "Tornado", "air")
    element("big_tornado", "Big Tornado", "air")
    element("bigger_tornado", "Bigger Tornado", "air")
    element("biggest_tornado", "Biggest Tornado", "air")
    element("biggerest_tornado", "Biggerest Tornado", "air")

    element("steam", "Steam", "air")
    element("dust", "Dust", "air")
    element("dust_bunny", "Dust Bunny", "air")
    element("ash", "Ash", "air")
    element("smoke", "Smoke", "air")
    element("smog", "Smog", "air")
    element("mist", "Mist", "air")
    element("fog", "Fog", "air")

    element("cloud", "Cloud", "air")
    element("rain_cloud", "Rain Cloud", "air")
    
    element("pollution", "Pollution", "air")
    element("dust_pollution", "Dust Pollution", "air")
    element("global_pollution", "Global Pollution", "air")
    element("acid_pollution", "Acid Pollution", "air")


    element("land", "Land", "earth")
    element("continent", "Continent", "earth")
    element("supercontinent", "Supercontinent", "earth")
    element("planet", "Planet", "earth")

    element("jupiter", "Jupiter", "earth")
    element("mars", "Mars", "earth")
    element("venus", "Venus", "earth")

    element("hill", "Hill", "earth")
    element("mountain", "Mountain", "earth")
    element("mountain_range", "Mountain Range", "earth")

    element("island", "Island", "earth")
    element("mud", "Mud", "earth")
    element("clay", "Clay", "earth")
    element("silt", "Silt", "earth")
    element("swamp", "Swamp", "earth")
    element("really_big_swamp", "Really Big Swamp", "earth")

    element("sand", "Sand", "earth")
    element("desert", "Desert", "earth")
    element("beach", "Beach", "earth")
    element("dune", "Dune", "earth")


    element("rock", "Rock", "rock")
    element("stone", "Stone", "rock")
    element("melted_rock", "Melted Rock", "rock")
    element("burnt_rock", "Burnt Stone", "rock")

    element("igneous_rock", "Igneous Rock", "rock")
    element("melted_igneous_rock", "Melted Igneous Rock", "rock")
    element("burnt_igneous_rock", "Burnt Igneous Rock", "rock")

    element("pebble", "Pebble", "rock")
    element("pumice", "Pumice", "rock")
    element("obsidian", "Obsidian", "rock")

    element("sandstone", "Sandstone", "rock")
    element("burnt_sandstone", "Burnt Sandstone", "rock")
    element("melted_sandstone", "Melted Sandstone", "rock")

    element("bricks", "Bricks", "rock")
    element("wall", "Wall", "rock")
    element("house", "House", "rock")
    element("village", "Village", "rock")


    element("big_fire", "Big Fire", "fire")
    element("bigger_fire", "Bigger Fire", "fire")
    element("biggest_fire", "Biggest Fire", "fire")
    element("biggerest_fire", "Biggerest Fire", "fire")

    element("hell", "Hell", "fire")

    element("lava", "Lava", "fire")
    element("lava_puddle", "Lava Puddle", "fire")
    element("lava_lake", "Lava Lake", "fire")
    element("lava_ocean", "Lava Ocean", "fire")

    element("volcano", "Volcano", "fire")
    element("super_volcano", "Supervolcano", "fire")

    element("glass", "Glass", "fire")


    element("puddle", "Puddle", "water")
    element("lake", "Lake", "water")
    element("ocean", "Ocean", "water")
    element("hurricane", "Hurricane", "water")

    element("oasis", "Oasis", "water")

    element("wave", "Wave", "water")
    element("big_wave", "Big Wave", "water")

    element("rain", "Rain", "water")
    element("acid_rain", "Acid Rain", "water")

    #

    alias("$base", ["air", "earth", "fire", "water"])
    alias("$airy", ["air", "steam", "mist"])
    alias("$cooling", ["air", "mist", "wind", "breeze"])
    alias("$big_wind", ["wind", "gale"])
    alias("$small_wind", ["wind", "breeze"])
    alias("$rocks", ["igneous_rock", "sandstone", "pebble", "pumice", "rock", "stone"])
    alias("$burnt_rocks", ["burnt_igneous_rock", "burnt_sandstone", "burnt_rock"])
    alias("$gen_rocks", ["pebble", "pumice", "rock", "stone"])
    alias("$polluting", ["smog", "smoke"])
    alias("$acid", ["acid_rain"])
    alias("$storm", ["storm", "wind_storm"])
    alias("$cloud", ["cloud", "rain_cloud"])
    alias("$water_bodies", ["lake", "ocean"])
    alias("$land", ["land", "continent", "supercontinent"])
    alias("$hill", ["hill", "mountain"])

    # alias combs

    combination("sand", "air", "$rocks")
    combination("igneous_rock", "lava", "$cooling")
    combination("burnt_rock", "$gen_rocks", "fire")
    combination("melted_rock", "$gen_burnt_rocks", "fire")
    combination("melted_rock", "$gen_rocks", "lava")
    combination("sand", "$rocks", "ocean")

    combination("pollution", "air", "$polluting")
    combination("pollution", "$polluting", "$polluting")
    combination("venus", "$polluting", "planet")
    combination("venus", "$acid", "planet")

    combination("dune", "$small_wind", "desert")
    combination("dune", "$small_wind", "beach")
    combination("storm", "rain", "$big_wind")
    combination("hurricane", "water", "$storm")
    combination("hurricane", "rain", "$storm")

    combination("wave", "wind", "$water_bodies")
    combination("big_wave", "gale", "$water_bodies")

    combination("volcano", "lava", "$hill")

    combination("acid_pollution", "pollution", "$acid")


    # burnt

    combination("burnt_igneous_rock", "igneous_rock", "fire")
    combination("burnt_sandstone", "sandstone", "fire")

    combination("melted_igneous_rock", "burnt_igneous_rock", "fire")
    combination("melted_igneous_rock", "igneous_rock", "lava")
    combination("melted_sandstone", "burnt_sandstone", "fire")
    combination("melted_sandstone", "sandstone", "lava")

    #

    combination("steam", "fire", "water")
    combination("mud", "water", "earth")
    combination("lava", "fire", "earth")
    combination("dust", "air", "earth")
    combination("smoke", "fire", "air")
    combination("mist", "air", "water")

    combination("land", "earth", "earth")
    combination("continent", "land", "land")
    combination("supercontinent", "continent", "continent")

    combination("island", "land", "ocean")
    combination("island", "earth", "ocean")

    combination("hill", "earth", "land")
    combination("mountain", "hill", "hill")
    combination("mountain_range", "mountain", "mountain")

    combination("planet", "supercontinent", "supercontinent")
    combination("planet", "continent", "ocean")

    combination("jupiter", "cloud", "planet")
    combination("jupiter", "storm", "planet")
    combination("mars", "desert", "planet")
    combination("venus", "pollution", "planet")
    combination("venus", "cloud", "planet")

    combination("puddle", "water", "water")
    combination("lake", "puddle", "puddle")
    combination("ocean", "lake", "lake")

    combination("lake", "water", "land")
    combination("lake", "water", "continent")

    combination("wind", "air", "air")
    combination("gale", "wind", "wind")
    combination("breeze", "wind", "air")
    combination("tornado", "gale", "gale")
    combination("big_tornado", "tornado", "tornado")
    combination("bigger_tornado", "big_tornado", "big_tornado")
    combination("biggest_tornado", "bigger_tornado", "bigger_tornado")
    combination("biggerest_tornado", "bigger_tornado", "biggest_tornado")

    combination("storm", "rain_cloud", "$big_wind")
    combination("wind_storm", "storm", "wind")

    combination("big_fire", "fire", "fire")
    combination("bigger_fire", "big_fire", "big_fire")
    combination("biggest_fire", "bigger_fire", "bigger_fire")
    combination("hell", "biggest_fire", "biggest_fire")

    combination("biggerest_fire", "biggerest_fire", "big_fire")

    combination("ash", "dust", "fire")

    combination("lava_puddle", "lava", "lava")
    combination("lava_lake", "lava_puddle", "lava_puddle")
    combination("lava_ocean", "lava_lake", "lava_lake")
    combination("hell", "lava_ocean", "lava_ocean")

    combination("super_volcano", "volcano", "volcano")

    combination("mist", "air", "steam")
    combination("cloud", "mist", "air")
    combination("cloud", "mist", "water")
    combination("rain_cloud", "cloud", "water")
    combination("rain", "rain_cloud", "water")
    combination("fog", "mist", "smoke")
    combination("smog", "smoke", "fog")
    combination("acid_rain", "pollution", "rain")

    combination("dust_pollution", "pollution", "dust")
    combination("global_pollution", "pollution", "planet")

    combination("dust_bunny", "dust", "dust")

    combination("swamp", "mud", "lake")
    combination("swamp", "mud", "land")
    combination("really_big_swamp", "mud", "ocean")

    combination("clay", "fire", "mud")
    combination("clay", "sand", "mud")
    combination("clay", "rock", "mud")
    combination("clay", "sandstone", "mud")
    combination("silt", "sand", "clay")

    combination("bricks", "fire", "clay")
    combination("wall", "bricks", "bricks")
    combination("house", "wall", "wall")
    combination("village", "house", "house")

    combination("pumice", "igneous_rock", "air")
    combination("stone", "pebble", "pebble")
    combination("obsidian", "water", "lava")
    combination("obsidian", "glass", "lava")

    combination("pebble", "water", "igneous_rock")
    combination("pebble", "water", "stone")
    combination("pebble", "water", "rock")

    combination("desert", "sand", "sand")
    combination("beach", "sand", "ocean")
    combination("beach", "desert", "ocean")
    combination("oasis", "lake", "desert")
    combination("sandstone", "earth", "sand")
    combination("sandstone", "rock", "sand")
    combination("glass", "sand", "fire")
    combination("sand", "pebble", "water")
