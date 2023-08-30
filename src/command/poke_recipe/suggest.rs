use super::models::{Bag, Recipe};

pub fn suggest_recipe(
    recipes: &[Recipe],
    category: String,
    capacity: u32,
    bag: &Bag,
) -> Vec<Recipe> {
    let mut available_recipes = recipes
        .iter()
        .filter(|recipe| cookable(recipe, category.clone(), capacity, bag))
        .cloned()
        .collect::<Vec<Recipe>>();
    available_recipes.sort_by(|a, b| b.energy.cmp(&a.energy));
    available_recipes
}

fn cookable(recipe: &Recipe, category: String, capacity: u32, bag: &Bag) -> bool {
    if recipe.category != category {
        return false;
    }
    for ingredient in recipe.ingredients.iter() {
        match bag.items.iter().find(|x| x.food.name == ingredient.name) {
            Some(food) => {
                if food.num < ingredient.num {
                    return false;
                }
            }
            None => return false,
        }
    }
    if capacity
        < recipe
            .ingredients
            .iter()
            .map(|ingredient| ingredient.num)
            .sum()
    {
        return false;
    }
    true
}

pub fn format_suggest(recipes: &[Recipe]) -> String {
    let mut text = String::new();
    for (i, recipe) in recipes.iter().enumerate() {
        text += &format!("### {}. {}\n", i + 1, recipe.name);
        text += &format!("- エナジー: {}\n", recipe.energy);
        text += "- レシピ \n";
        for ingredient in &recipe.ingredients {
            text += &format!("  - {}: {}個\n", ingredient.name, ingredient.num);
        }
    }
    text
}
