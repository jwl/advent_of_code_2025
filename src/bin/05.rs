advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    // Parse input into fresh ranges, and then ingredient IDs
    let mut input_parts = input.split("\n\n");

    let ranges_input = input_parts.next().unwrap_or("").to_string();
    let ingredients = input_parts.next().unwrap_or("").to_string();

    let known_fresh = populate_known_fresh(&ranges_input);

    let mut fresh_ingredients: Vec<u64> = Vec::new();
    for ingredient in ingredients.trim().lines() {
        let id: u64 = ingredient.parse().unwrap();
        if known_fresh.contains(&id) {
            fresh_ingredients.push(id);
        }
    }

    println!("final fresh ingredients is:\n{:?}", fresh_ingredients);

    Some(fresh_ingredients.len() as u64)
}

pub fn populate_known_fresh(fresh_ranges: &str) -> Vec<u64> {
    let mut known_fresh: Vec<u64> = Vec::new();

    for line in fresh_ranges.trim().lines() {
        let (low, high) = line
            .split_once('-')
            .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
            .unwrap();

        for i in low..high + 1 {
            known_fresh.push(i);
        }
    }

    known_fresh
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
