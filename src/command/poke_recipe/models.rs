use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
pub struct Food {
    pub name: String,
    pub energy: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub category: String,
    pub energy: u32,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub num: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bag {
    pub items: Vec<BagItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BagItem {
    pub food: Food,
    pub num: u32,
}
