use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    color::HexColor,
    parser::{Category, Combination, CombinationWithElement, Element, Stmt},
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
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementData {
    pub name: String,
    pub category: String,
    pub color: HexColor,
    pub tier: i32,
    pub used_in: i32,
}

pub fn get_lowest_combinations(data: &GameData) -> Vec<((String, String), i32)> {
    let elem_ids: Vec<_> = data.elements.keys().collect();
    let mut all_combinations: Vec<(String, String)> = Vec::new();

    for i in 0..elem_ids.len() {
        // start iter at i (i == j) to allow self pairs but not duplicates
        for j in i..elem_ids.len() {
            all_combinations.push((elem_ids[i].clone(), elem_ids[j].clone()));
        }
    }

    let all_combinations = all_combinations.iter().filter(|(elem1, elem2)| {
        !(data.combinations.contains_key(&format!("{elem1}|{elem2}"))
            || data.combinations.contains_key(&format!("{elem2}|{elem1}")))
    });

    let mut combinations_with_tier: Vec<((String, String), i32)> = Vec::new();

    let mut computed_tiers: HashMap<String, i32> = HashMap::from([
        ("Air".into(), 1),
        ("Earth".into(), 1),
        ("Fire".into(), 1),
        ("Water".into(), 1),
    ]);

    for (elem1, elem2) in all_combinations {
        let t1 = compute_tier(elem1, data, &mut computed_tiers);
        let t2 = compute_tier(elem2, data, &mut computed_tiers);
        let tier = std::cmp::max(t1, t2) + 1;
        combinations_with_tier.push(((elem1.clone(), elem2.clone()), tier));
    }

    combinations_with_tier.sort_by_key(|(_, tier)| *tier);

    combinations_with_tier
}

pub fn stmts_to_data(stmts: Vec<Stmt>) -> Result<GameData, String> {
    let mut data = GameData::new();

    for stmt in &stmts {
        #[allow(clippy::single_match)]
        match stmt {
            Stmt::Category(category) => {
                let (id, cat_data) = category_to_data(category, &data)?;
                data.categories.insert(id, cat_data);
            }
            _ => {}
        }
    }

    for stmt in &stmts {
        match stmt {
            Stmt::Element(element) => {
                let (id, elem_data) = element_to_data(element, &data)?;
                data.elements.insert(id, elem_data);
            }
            Stmt::CombinationWithElement(CombinationWithElement {
                element,
                combination: _,
            }) => {
                let (id, elem_data) = element_to_data(element, &data)?;
                data.elements.insert(id, elem_data);
            }
            _ => {}
        }
    }

    for stmt in &stmts {
        match stmt {
            Stmt::Combination(combination) => {
                let (id, result) = combination_to_data(combination, &data)?;
                data.combinations.insert(id, result);
            }
            Stmt::CombinationWithElement(CombinationWithElement {
                element: _,
                combination,
            }) => {
                let (id, result) = combination_to_data(combination, &data)?;
                data.combinations.insert(id, result);
            }
            _ => {}
        }
    }

    let cat_keys: Vec<_> = data.categories.keys().cloned().collect();
    for category_id in cat_keys {
        let amount: i32 = data
            .elements
            .iter()
            .filter(|(_id, elem)| elem.category == category_id)
            .count() as i32;
        data.categories.get_mut(&category_id).unwrap().amount = amount;
    }

    let elem_keys: Vec<_> = data.elements.keys().cloned().collect();
    for elem_id in elem_keys {
        let used = data
            .combinations
            .iter()
            .filter(|(comb_id, _res)| {
                elems_from_comb_id(comb_id).0 == elem_id || elems_from_comb_id(comb_id).1 == elem_id
            })
            .count() as i32;
        data.elements.get_mut(&elem_id).unwrap().used_in = used;
    }

    let mut computed_tiers: HashMap<String, i32> = HashMap::from([
        ("Air".into(), 1),
        ("Earth".into(), 1),
        ("Fire".into(), 1),
        ("Water".into(), 1),
    ]);

    let elem_keys: Vec<_> = data.elements.keys().cloned().collect();
    for elem_id in elem_keys {
        let tier = compute_tier(&elem_id, &data, &mut computed_tiers);
        data.elements.get_mut(&elem_id).unwrap().tier = tier;
    }

    Ok(data)
}

fn compute_tier(id: &str, data: &GameData, computed: &mut HashMap<String, i32>) -> i32 {
    if let Some(tier) = computed.get(id) {
        return *tier;
    }

    let combinations = data
        .combinations
        .iter()
        .filter_map(|(comb_id, result)| if result == id { Some(comb_id) } else { None });

    let tiers = combinations.map(|comb_id| {
        let (c1, c2) = elems_from_comb_id(comb_id);

        let t1 = compute_tier(&c1, data, computed);
        let t2 = compute_tier(&c2, data, computed);

        std::cmp::max(t1, t2) + 1
    });

    let tier = tiers.min().unwrap_or(1000); // random really high number lmao
    computed.insert(id.to_string(), tier);
    tier
}

fn elems_from_comb_id(comb_id: &str) -> (String, String) {
    let (e1, e2) = comb_id.split_once('|').unwrap();
    debug_assert!(!e2.contains('|'));

    (e1.to_owned(), e2.to_owned())
}

fn combination_to_data(
    combination: &Combination,
    data: &GameData,
) -> Result<(String, String), String> {
    let Combination { a, b, result } = combination;
    let a_id = a;
    let b_id = b;
    let res_id = result;

    if !data.elements.contains_key(a_id) {
        return Err(format!("Element {a} is not declared"));
    }
    if !data.elements.contains_key(b_id) {
        return Err(format!("Element {b} is not declared"));
    }
    if !data.elements.contains_key(res_id) {
        return Err(format!("Element {result} is not declared"));
    }

    let id = format!("{a_id}|{b_id}");

    if data.combinations.contains_key(&id) {
        return Err(format!("Combination {id} already declared"));
    }

    Ok((id, res_id.to_owned()))
}

fn category_to_data(
    category: &Category,
    data: &GameData,
) -> Result<(String, CategoryData), String> {
    let Category { name, color } = category;

    let id = name;
    if data.categories.contains_key(id) {
        return Err(format!("Category {name} already declared"));
    }

    Ok((
        id.to_owned(),
        CategoryData {
            name: name.to_owned(),
            color: color
                .clone()
                .unwrap_or(HexColor::from_str("#000000").unwrap()),
            amount: 0,
        },
    ))
}

fn element_to_data(element: &Element, data: &GameData) -> Result<(String, ElementData), String> {
    let Element { name, category } = element;

    let id = name;
    let category_id = category;
    if data.elements.contains_key(id) {
        return Err(format!("Element {name} already declared"));
    }
    if !data.categories.contains_key(category_id) {
        return Err(format!("Category {category} not yet declared"));
    }

    let color = data.categories.get(category_id).unwrap().color.clone();

    Ok((
        id.to_owned(),
        ElementData {
            name: name.to_owned(),
            category: category_id.to_owned(),
            color,
            tier: 1000,
            used_in: 0,
        },
    ))
}
