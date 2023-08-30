use anyhow::bail;
use regex::Regex;

use super::{
    models::{Bag, BagItem, Food},
    FOODS,
};

static NUM_RE: &str = "x([1-9][0-9]{2}|[1-9][0-9]|[1-9])";

pub fn extract_bag_items(text: String) -> anyhow::Result<Bag> {
    let foods: Vec<Food> = FOODS
        .iter()
        .filter(|food| text.contains(&food.name))
        .cloned()
        .collect();

    let regex = Regex::new(NUM_RE).unwrap();
    let nums: Vec<u32> = regex
        .captures_iter(&text)
        .map(|cap| cap[1].parse::<u32>().unwrap())
        .collect();
    // println!("{:#?}", foods);
    // println!("{:#?}", nums);
    if foods.len() != nums.len() {
        return bail!("OCR failed");
    }
    Ok(Bag {
        items: foods
            .iter()
            .zip(nums.iter())
            .map(|(f, &n)| BagItem {
                food: f.clone(),
                num: n,
            })
            .collect(),
    })
}
