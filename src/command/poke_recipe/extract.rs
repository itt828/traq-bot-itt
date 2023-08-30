use anyhow::bail;
use regex::Regex;

use super::{
    models::{Bag, BagItem},
    FOODS,
};

static NUM_RE: &str = "x([1-9][0-9]{2}|[1-9][0-9]|[1-9])";

pub fn extract_bag_items(text: String) -> anyhow::Result<Bag> {
    let food_re = {
        let mut s = String::new();
        s += "(";
        s += &FOODS
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<_>>()
            .join("|");
        s += ")";
        s
    };
    let food_regex_built = Regex::new(&food_re)?;
    let foods = food_regex_built
        .captures_iter(&text)
        .map(|cap| {
            let food_name = cap[1].to_string();
            let food = FOODS
                .iter()
                .filter(|f| f.name == food_name)
                .collect::<Vec<_>>();
            if food.len() != 1 {
                Err("error")
            } else {
                Ok(food[0].clone())
            }
        })
        .collect::<Result<Vec<_>, &str>>()
        .unwrap();

    let regex = Regex::new(NUM_RE).unwrap();
    let nums: Vec<u32> = regex
        .captures_iter(&text)
        .map(|cap| cap[1].parse::<u32>().unwrap())
        .collect();
    // println!("{:#?}", foods);
    // println!("{:#?}", nums);
    if foods.len() != nums.len() {
        bail!("OCR failed")
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
