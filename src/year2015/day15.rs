//! # 2015 day 15 - Science for Hungry People
//!
//! Generates the different quantity combinations through recursion, with just an additional
//! constraint for part 2. Calculates the results for both parts in one go, since all combinations
//! anyhow need to be explored for both.
//!
//! Part 1 asks for the best score over every way of splitting 100 teaspoons between the
//! ingredients, and part 2 for the best score among the splits that land on exactly 500 calories.
//! A score is the product of the four properties, each one clamped at zero before it is multiplied
//! in, so a recipe that turns negative on any single property scores nothing at all rather than
//! scoring negatively.
//!
//! That clamp is also why the running maximum starts at 0 and is not merely a placeholder. Every
//! score is a product of non-negative terms, so no score can be negative and 0 is both reachable
//! and a true floor. Day 13 seeded an accumulator with 0 and that was a real bug, because a
//! seating total can be negative and the search found it, then discarded it. Here there is nothing
//! valid below 0 for the accumulator to hide.
//!
//! The last ingredient is never chosen. Whatever is left over once the others have been given
//! their quantities is assigned to it, which is what keeps the search from walking the same split
//! several times over.
use crate::aoc_util::error::*;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

const TOTAL_INGREDIENTS: i64 = 100;
const CALORIE_MAX: i64 = 500;

#[derive(Debug)]
struct Ingredient {
    amounts: [i8; 5],
}

impl FromStr for Ingredient {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            [
                _,
                "capacity",
                cap,
                "durability",
                dur,
                "flavor",
                fla,
                "texture",
                tex,
                "calories",
                cal,
            ] => Ok(Self {
                amounts: [
                    cap.trim_end_matches(',').parse()?,
                    dur.trim_end_matches(',').parse()?,
                    fla.trim_end_matches(',').parse()?,
                    tex.trim_end_matches(',').parse()?,
                    cal.trim_end_matches(',').parse()?,
                ],
            }),
            _ => Err(AocError::Invalid(format!("invalid ingredient: {s}"))),
        }
    }
}

struct InputData {
    ingredients: Vec<Ingredient>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            ingredients: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_recipe(&self, remaining: i64, quantities: &mut Vec<i64>) -> (i64, i64) {
        if quantities.len() == self.ingredients.len() - 1 {
            // We are at the last ingredient - assign the remainder
            let totals = quantities
                .iter()
                .copied()
                .chain(std::iter::once(remaining))
                .zip(&self.ingredients)
                .fold([0i64; 5], |mut totals, (q, ingredient)| {
                    for (total, &amount) in totals.iter_mut().zip(ingredient.amounts.iter()) {
                        *total += q * i64::from(amount);
                    }
                    totals
                });
            let score = totals[..4]
                .iter()
                .fold(1i64, |acc, &total| acc * total.max(0));

            // Calorie check for part 2
            let cal_score = if totals[4] == CALORIE_MAX { score } else { 0 };

            (score, cal_score)
        } else {
            // Recursion by trying all possible quantities for the next ingredient
            let mut max_score_p1 = 0;
            let mut max_score_p2 = 0;
            for q in 0..remaining + 1 {
                quantities.push(q);
                let scores = self.solve_recipe(remaining - q, quantities);
                max_score_p1 = max_score_p1.max(scores.0);
                max_score_p2 = max_score_p2.max(scores.1);
                quantities.pop();
            }

            (max_score_p1, max_score_p2)
        }
    }

    fn solve(&self) -> (i64, i64) {
        let mut v = Vec::new();
        self.solve_recipe(TOTAL_INGREDIENTS, &mut v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 62842880);
        assert_eq!(p2, 57600000);
    }
}
