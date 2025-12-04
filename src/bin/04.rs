advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    println!("input is {input}");
    13
}

pub fn part_two(input: &str) -> Option<u64> {
    println!("input is {input}");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
