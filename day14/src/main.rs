use aoc::utils::get_lines;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Resource {
    name: String,
    quantity: u64,
}

impl Resource {
    fn from(raw_string: &str) -> Self {
        if let Ok((quantity, name)) = scan_fmt!(raw_string.trim(), "{d} {s}", u64, String) {
            Resource { name, quantity }
        } else {
            panic!("Bad input: {}", raw_string);
        }
    }
}

fn get_required_ore_v1(
    resource: &Resource,
    resource_lookup: &HashMap<String, Resource>,
    recipes: &HashMap<Resource, Vec<Resource>>,
    excess: &mut HashMap<String, u64>,
) -> u64 {
    let mut result: u64 = 0;
    let recipe = recipes.get(resource).unwrap();

    for ingredient in recipe.iter() {
        if excess.get(&ingredient.name).unwrap_or(&0) > &ingredient.quantity {
            excess.insert(
                ingredient.name.clone(),
                excess.get(&ingredient.name).unwrap() - ingredient.quantity,
            );
        } else {
            if ingredient.name == "ORE" {
                return ingredient.quantity;
            }

            let standard_resource = resource_lookup.get(&ingredient.name).unwrap();

            let available_excess = *excess.get(&ingredient.name).unwrap_or(&0);
            let ingredient_quantity_after_excess = match available_excess > 0 {
                true => {
                    excess.insert(ingredient.name.clone(), 0);
                    ingredient.quantity - available_excess
                }
                false => ingredient.quantity,
            };

            let required_units = (ingredient_quantity_after_excess / standard_resource.quantity)
                + match ingredient_quantity_after_excess % standard_resource.quantity == 0 {
                    true => 0,
                    false => 1,
                };

            let excess_ingredient_quantity =
                (required_units * standard_resource.quantity) - ingredient_quantity_after_excess;

            if excess_ingredient_quantity > 0 {
                excess.insert(
                    ingredient.name.clone(),
                    excess.get(&ingredient.name).unwrap_or(&0) + excess_ingredient_quantity,
                );
            }

            for i in 0..required_units {
                result += get_required_ore_v1(&standard_resource, resource_lookup, recipes, excess);
            }
        }
    }

    result
}

fn get_required_ore_v2(
    resource: &Resource,
    resource_lookup: &HashMap<String, Resource>,
    recipes: &HashMap<Resource, Vec<Resource>>,
) -> u64 {
    let mut required: HashMap<String, u64> = HashMap::new();
    let mut have: HashMap<String, u64> = HashMap::new();
    required.insert(String::from("FUEL"), resource.quantity);

    loop {
        if required.len() == 1 {
            if let Some(v) = required.get("ORE") {
                break;
            }
        }

        // Dummy value; it won't be used
        let mut resource_to_expand: Resource = resource.clone();
        for (name, quantity) in required.iter() {
            if name == "ORE" {
                continue;
            } else {
                resource_to_expand = Resource {
                    name: name.to_string(),
                    quantity: *quantity,
                };
                break;
            }
        }

        let standard_resource = resource_lookup.get(&resource_to_expand.name).unwrap();
        let recipe = recipes.get(standard_resource).unwrap();

        let mut required_copies_of_standard_resource =
            resource_to_expand.quantity / standard_resource.quantity;

        if resource_to_expand.quantity % standard_resource.quantity != 0 {
            required.remove(&standard_resource.name);
            required_copies_of_standard_resource += 1;
            have.insert(
                standard_resource.name.clone(),
                standard_resource.quantity
                    - resource_to_expand.quantity % standard_resource.quantity,
            );
        } else {
            required.remove(&standard_resource.name);
        }

        for ingredient in recipe.iter() {
            let current_amount = have.get(&ingredient.name).unwrap_or(&0);
            if *current_amount < ingredient.quantity * required_copies_of_standard_resource {
                required.insert(
                    ingredient.name.clone(),
                    required.get(&ingredient.name).unwrap_or(&0)
                        + (ingredient.quantity * required_copies_of_standard_resource)
                        - current_amount,
                );

                have.remove(&ingredient.name);
            } else {
                have.insert(
                    ingredient.name.clone(),
                    current_amount - ingredient.quantity * required_copies_of_standard_resource,
                );
            }
        }
    }

    *required.get("ORE").unwrap()
}

fn main() {
    let lines = get_lines("input.txt").clone();

    let mut resource_lookup: HashMap<String, Resource> = HashMap::new();
    let mut recipes: HashMap<Resource, Vec<Resource>> = HashMap::new();

    // Store recipes
    for recipe in lines.iter() {
        let recipe_parts: Vec<&str> = recipe.split(" => ").collect();
        let ingredients: Vec<Resource> = recipe_parts[0]
            .split(",")
            .map(|x| Resource::from(x))
            .collect();
        let result: Resource = Resource::from(recipe_parts[1]);

        resource_lookup.insert(result.name.clone(), result.clone());
        recipes.insert(result, ingredients);
    }

    // Part 1
    {
        let required_ore: u64 = get_required_ore_v2(
            &Resource {
                name: "FUEL".to_string(),
                quantity: 1,
            },
            &resource_lookup,
            &recipes,
        );

        println!("{}", required_ore);
    }

    // Part 2
    {
        let maximum_ore: u64 = 1000000000000;

        let mut required_ore: u64 = 0;
        let mut desired_fuel = 1000000;
        let mut step_size = 5000000;
        let mut checkpoint = 1;

        loop {
            desired_fuel = checkpoint;
            loop {
                // Find the recipe for FUEL
                required_ore = get_required_ore_v2(
                    &Resource {
                        name: "FUEL".to_string(),
                        quantity: desired_fuel,
                    },
                    &resource_lookup,
                    &recipes,
                );

                if required_ore as u64 >= maximum_ore {
                    break;
                } else {
                    checkpoint = desired_fuel;
                    desired_fuel += step_size;
                }
            }

            if step_size == 1 {
                println!("{}", checkpoint);
                break;
            } else {
                step_size /= 2;
            }
        }
    }
}
