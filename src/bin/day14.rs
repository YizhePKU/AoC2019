use petgraph::{
    algo::toposort,
    // dot::{Config, Dot},
    graphmap::DiGraphMap,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Recipe {
    ingredients: HashMap<String, usize>,
    quantity: usize,
}

fn parse_input() -> HashMap<String, Recipe> {
    let input = std::fs::read("data/day14").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut recipes = HashMap::new();
    for line in input.split_terminator("\r\n") {
        let (lhs, rhs) = line.split_once(" => ").unwrap();

        let mut ingredients = HashMap::new();
        for ingredient in lhs.split_terminator(", ") {
            let (quantity, name) = ingredient.split_once(" ").unwrap();
            ingredients.insert(name.to_string(), quantity.parse().unwrap());
        }

        let (quantity, product) = rhs.split_once(" ").unwrap();
        let recipe = Recipe {
            ingredients,
            quantity: quantity.parse().unwrap(),
        };
        recipes.insert(product.to_string(), recipe);
    }
    recipes
}

fn ore_required(recipes: &HashMap<String, Recipe>, fuel_requested: usize) -> usize {
    let mut graph: DiGraphMap<&String, ()> = DiGraphMap::new();
    for (product, recipe) in recipes {
        for (ingrident, _) in &recipe.ingredients {
            graph.add_edge(ingrident, product, ());
        }
    }

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let order = toposort(&graph, None).unwrap();

    let mut requirements: HashMap<String, usize> = HashMap::new();
    requirements.insert("FUEL".to_string(), fuel_requested);
    for product in order.into_iter().rev() {
        if let Some(recipe) = recipes.get(product) {
            if let Some(&quantity) = requirements.get(product) {
                assert!(quantity > 0);
                requirements.remove(product);

                let multiplier = (quantity + recipe.quantity - 1) / recipe.quantity;
                for (ingredient, &quantity) in &recipe.ingredients {
                    *requirements.entry(ingredient.to_string()).or_default() +=
                        quantity * multiplier;
                }
            }
        }
    }
    requirements["ORE"]
}

fn main() {
    let recipes = parse_input();

    let part1 = ore_required(&recipes, 1);
    dbg!(part1);

    // search within [low, high)
    let mut low = 1;
    let mut high = 100000000;
    while low + 1 < high {
        let mid = (low + high) / 2;
        if ore_required(&recipes, mid) <= 1000000000000 {
            low = mid;
        } else {
            high = mid;
        }
    }
    let part2 = low;
    dbg!(part2);
}
