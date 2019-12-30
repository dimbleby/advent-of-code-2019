use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ingredient {
    chemical: String,
    amount: usize,
}

impl Ingredient {
    fn new(chemical: String, amount: usize) -> Self {
        Self { chemical, amount }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ParseIngredientError;

impl From<std::num::ParseIntError> for ParseIngredientError {
    fn from(_err: std::num::ParseIntError) -> Self {
        Self
    }
}

impl FromStr for Ingredient {
    type Err = ParseIngredientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "12 ABC"
        let mut words = s.split(' ');
        let amount = words.next().ok_or(ParseIngredientError)?;
        let amount = amount.parse()?;
        let chemical = words.next().ok_or(ParseIngredientError)?;
        let ingredient = Ingredient::new(chemical.to_string(), amount);
        Ok(ingredient)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Recipe {
    inputs: Vec<Ingredient>,
    output: Ingredient,
}

impl Recipe {
    fn new(inputs: Vec<Ingredient>, output: Ingredient) -> Self {
        Self { inputs, output }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ParseRecipeError;

impl From<ParseIngredientError> for ParseRecipeError {
    fn from(_err: ParseIngredientError) -> Self {
        Self
    }
}

impl FromStr for Recipe {
    type Err = ParseRecipeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "12 ABC, 34 DEF => 56 GHI"
        let mut segments = s.split("=>");

        let inputs = segments.next().ok_or(ParseRecipeError)?;
        let inputs = inputs
            .split(',')
            .map(|i| i.trim().parse())
            .collect::<Result<_, _>>()?;

        let output = segments.next().ok_or(ParseRecipeError)?;
        let output = output.trim().parse()?;

        let recipe = Recipe::new(inputs, output);
        Ok(recipe)
    }
}

pub(crate) fn day14() {
    let mut recipes = HashMap::new();
    let input = File::open("data/day14.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    for line in buffered.lines().map(|line| line.unwrap()) {
        let recipe: Recipe = line.parse().expect("Couldn't parse recipe");
        recipes.insert(recipe.output.chemical.to_owned(), recipe);
    }

    // Part one.
    let mut needed = HashMap::new();
    needed.insert("FUEL".to_owned(), 1);
    let ore_needed = produce(&recipes, needed);
    println!("Part one answer is: {}", ore_needed);

    // Part two.
    let mut lower = 0;
    let mut upper = 10_000_000;
    let mut middle = 0;
    while lower <= upper {
        middle = (lower + upper) / 2;
        let mut needed = HashMap::new();
        needed.insert("FUEL".to_owned(), middle);
        let ore_needed = produce(&recipes, needed);
        if ore_needed > 1_000_000_000_000 {
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }
    println!("Part two answer is: {}", middle);
}

fn produce(recipes: &HashMap<String, Recipe>, mut needed: HashMap<String, usize>) -> u64 {
    let mut surplus = HashMap::new();
    let mut ore_used = 0u64;
    while !needed.is_empty() {
        let (output, needed_amount) = needed.iter().next().unwrap();
        let output = output.clone();
        let needed_amount = *needed_amount;
        let recipe = recipes.get(&output).unwrap();
        let multiplier = (needed_amount + recipe.output.amount - 1) / recipe.output.amount;
        for ingredient in &recipe.inputs {
            let mut ingredient_amount = multiplier * ingredient.amount;
            if ingredient.chemical == "ORE" {
                ore_used += ingredient_amount as u64;
            } else {
                surplus
                    .entry(ingredient.chemical.clone())
                    .and_modify(|surplus_amount| {
                        if *surplus_amount > ingredient_amount {
                            *surplus_amount -= ingredient_amount;
                            ingredient_amount = 0;
                        } else {
                            ingredient_amount -= *surplus_amount;
                            *surplus_amount = 0;
                        }
                    });
                if ingredient_amount > 0 {
                    let amount = needed.entry(ingredient.chemical.clone()).or_insert(0);
                    *amount += ingredient_amount;
                }
            }
        }
        needed.remove(&output);
        let produced_amount = multiplier * recipe.output.amount;
        surplus.insert(output, produced_amount - needed_amount);
    }
    ore_used
}
