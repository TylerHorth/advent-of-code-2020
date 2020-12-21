use std::collections::{HashMap, HashSet};

use pest_derive::Parser;
use pest::{Parser, iterators::Pair};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Parser)]
#[grammar = "grammars/day21.pest"]
struct FoodParser;

fn collect(pair: Pair<Rule>) -> Vec<String> {
    pair
        .into_inner()
        .map(|p| p.as_str().to_string())
        .collect()
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut result = Vec::new();

    for food in FoodParser::parse(Rule::all, input).unwrap() {
        if matches!(food.as_rule(), Rule::EOI) {
            break
        }

        let (ingredients, allergens) = food.into_inner().next_tuple().unwrap();

        result.push((collect(ingredients), collect(allergens)))
    }

    result
}

fn find_allergens(food: &Vec<(Vec<String>, Vec<String>)>) -> HashMap<&String, &String> {
    let mut potential: HashMap<&String, HashSet<&String>> = HashMap::new();

    for (ingredients, allergens) in food {
        let ingredients: HashSet<&String> = ingredients.iter().collect();

        for allergen in allergens {
            potential.entry(allergen).and_modify(|set| *set = &*set & &ingredients ).or_insert(ingredients.clone());
        }
    }

    let mut allergen_map = HashMap::new();

    while let Some((&allergen, set)) = potential.iter().find(|(_, set)| set.len() == 1) {
        let ingredient = *set.iter().exactly_one().unwrap();
        allergen_map.insert(allergen, ingredient);
        potential.values_mut().for_each(|set| { set.remove(ingredient); });
    }

    allergen_map
}

#[aoc(day21, part1)]
fn part1(food: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    let allergen_map = find_allergens(food);
    let ingredients: HashSet<_> = allergen_map.values().collect();

    food
        .iter()
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|ingredient| !ingredients.contains(&ingredient))
        .count()
}

#[aoc(day21, part2)]
fn part2(food: &Vec<(Vec<String>, Vec<String>)>) -> String {
    let allergen_map = find_allergens(food);

    allergen_map
        .iter()
        .sorted()
        .map(|(_, ingredient)| ingredient)
        .join(",")
}
