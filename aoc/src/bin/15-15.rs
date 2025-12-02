puzzle_lib::setup!(title = "Science for Hungry People");

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse_input(input: &str) -> HashMap<&str, Ingredient> {
    parse!(input => {
        [ingredients split on '\n' into (HashMap<_, _>) with
            {
                name
                ": capacity "
                [capacity as isize]
                ", durability "
                [durability as isize]
                ", flavor "
                [flavor as isize]
                ", texture "
                [texture as isize]
                ", calories "
                [calories as isize]
            }
            => (
                name,
                Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            )
        ]
    } => ingredients)
}

#[inline]
fn add_to_sum(ingredient: &Ingredient, count: isize, sums: &mut [isize; 4]) {
    sums[0] += count * ingredient.capacity;
    sums[1] += count * ingredient.durability;
    sums[2] += count * ingredient.flavor;
    sums[3] += count * ingredient.texture;
}

#[inline]
fn find_optimal(ingredients: &[Ingredient], left: isize, sums: [isize; 4]) -> usize {
    if left == 0 {
        return sums
            .into_iter()
            .map(|v| isize::max(0, v) as usize)
            .reduce(|a, b| a * b)
            .unwrap();
    }

    let ingredient = &ingredients[0];
    let ingredients = &ingredients[1..];

    if ingredients.is_empty() {
        let mut sums = sums;
        add_to_sum(ingredient, left, &mut sums);
        return find_optimal(&[], 0, sums);
    }

    (0..=left)
        .map(|count| {
            let mut sums = sums;
            add_to_sum(ingredient, count, &mut sums);
            find_optimal(ingredients, left - count, sums)
        })
        .max()
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    let ingredients = parse_input(input);
    find_optimal(&ingredients.into_values().collect_vec(), 100, [0, 0, 0, 0])
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 62_842_880)]
    static EXAMPLE_INPUT: &str = "
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    ";
}
