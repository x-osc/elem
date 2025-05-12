use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for HexColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);

        if hex.len() != 6 {
            return Err("Hex color must be 6 characters long".into());
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;

        Ok(HexColor { r, g, b })
    }
}

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
