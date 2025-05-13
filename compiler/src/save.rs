use crate::data::GameData;

pub struct JsonFiles {
    pub elements: String,
    pub categories: String,
    pub combinations: String,
}

pub fn data_to_files(data: GameData) -> Result<JsonFiles, serde_json::Error> {
    let elements = serde_json::to_string(&data.elements)?;
    let categories = serde_json::to_string(&data.categories)?;
    let combinations = serde_json::to_string(&data.combinations)?;

    Ok(JsonFiles {
        elements,
        categories,
        combinations,
    })
}
