import re


def verify_hex_color(value: str):
    if not value.startswith("#"):
        raise ValueError("Color must start with '#'")
    if len(value) != 7:
        raise ValueError("Color must have 6 hexadecimal digits")
    hex_part = value[1:]
    if not all(c in "0123456789abcdefABCDEF" for c in hex_part):
        raise ValueError("Invalid non-hexadecimal digits in color")
    return value


def verify_id(id: str):
    if len(id) == 0:
        raise ValueError("Id must not be empty")
    if not bool(re.match(r"^[a-zA-Z0-9_-]+$", id)):
        raise ValueError("Id must be alphanumeric")
    return id
