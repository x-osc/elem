use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    color::HexColor,
    parser::{Category, Combination, CombinationWithElement, Element, Meta, Stmt},
};

#[derive(Debug)]
pub struct GameData {
    pub elements: HashMap<String, ElementData>,
    pub categories: HashMap<String, CategoryData>,
    pub combinations: HashMap<String, String>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            categories: HashMap::new(),
            combinations: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryData {
    pub name: String,
    pub color: HexColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementData {
    pub name: String,
    pub category: String,
    pub color: HexColor,
    pub tier: i32,
}

pub fn stmts_to_data(stmts: Vec<Stmt>) -> Result<GameData, String> {
    let mut data = GameData::new();

    for stmt in stmts {
        match stmt {
            Stmt::Meta(Meta { prop: _, val: _ }) => (),
            Stmt::Category(category) => {
                let (id, cat_data) = category_to_data(category, &data)?;
                data.categories.insert(id, cat_data);
            }
            Stmt::Element(element) => {
                let (id, elem_data) = element_to_data(element, &data)?;
                data.elements.insert(id, elem_data);
            }
            Stmt::Combination(combination) => {
                let (id, result) = combination_to_data(combination, &data)?;
                data.combinations.insert(id, result);
            }
            Stmt::CombinationWithElement(CombinationWithElement {
                element,
                combination,
            }) => {
                let (id, elem_data) = element_to_data(element, &data)?;
                data.elements.insert(id, elem_data);

                let (id, result) = combination_to_data(combination, &data)?;
                data.combinations.insert(id, result);
            }
        }
    }

    Ok(data)
}

fn combination_to_data(
    combination: Combination,
    data: &GameData,
) -> Result<(String, String), String> {
    let a_id = combination.a;
    let b_id = combination.b;
    let res_id = combination.result;

    if !data.elements.contains_key(&a_id) {
        return Err("Element a is not declared".into());
    }
    if !data.elements.contains_key(&b_id) {
        return Err("Element b is not declared".into());
    }
    if !data.elements.contains_key(&res_id) {
        return Err("Result element is not declared".into());
    }

    let id = format!("{a_id}|{b_id}");

    Ok((id, res_id))
}

fn category_to_data(category: Category, data: &GameData) -> Result<(String, CategoryData), String> {
    let Category { name } = category;

    let id = name.clone();
    if data.categories.contains_key(&id) {
        return Err("Category already declared".into());
    }

    Ok((
        id,
        CategoryData {
            name,
            color: HexColor::from_str("#555555").unwrap(),
        },
    ))
}

fn element_to_data(element: Element, data: &GameData) -> Result<(String, ElementData), String> {
    let Element { name, category } = element;

    let id = name.clone();
    let category_id = category.clone();
    if data.elements.contains_key(&id) {
        return Err("Element already declared".into());
    }
    if !data.categories.contains_key(&category_id) {
        return Err("Category not yet declared".into());
    }

    Ok((
        id,
        ElementData {
            name,
            category: category_id,
            color: HexColor::from_str("#555555").unwrap(),
            tier: 1,
        },
    ))
}
