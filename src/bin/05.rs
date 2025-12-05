advent_of_code::solution!(5);

pub fn populate_fresh_map(fresh_ranges: &str) -> HashMap5

pub fn part_one(input: &str) -> Option<u64> {
    // Parse input into fresh ranges, and then ingredient IDs
    let mut input_parts = input.split("\n\n");

    let fresh_ranges = input_parts.next().unwrap_or("").to_string();
    let ingredient_ids = input_parts.next().unwrap_or("").to_string();
    None
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
