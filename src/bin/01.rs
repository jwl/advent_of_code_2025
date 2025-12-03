advent_of_code::solution!(1);

use std::num::ParseIntError;

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;
    let mut pointed_numbers: Vec<i32> = Vec::new();
    pointed_numbers.push(dial);

    for line in input.lines() {
        let line = line.trim();
        dial = parse_instruction_part1(dial, line).unwrap();
        pointed_numbers.push(dial);
    }

    let zero_count = pointed_numbers.iter().filter(|&&x| x == 0).count();

    Some(zero_count as u64)
}

pub fn parse_instruction_part1(current_number: i32, instruction: &str) -> Result<i32, String> {
    if instruction.len() < 2 {
        return Err(format!("Instruction was too short: '{}'", instruction));
    }

    let (direction, distance_str) = instruction.split_at(1);
    let distance: i32 = distance_str
        .parse()
        .map_err(|e: ParseIntError| format!("Invalid number '{}': {}", distance_str, e))?;

    let result = match direction {
        "L" => ((current_number - distance) % 100 + 100) % 100,
        "R" => ((current_number + distance) % 100 + 100) % 100,
        _ => return Err(format!("Invalid direction '{}'", direction)),
    };

    Ok(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial: u64 = 50;
    let mut pointed_numbers: Vec<u64> = Vec::new();
    pointed_numbers.push(dial);
    let mut rotated_zeroes: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        let zeroes: u64;

        (dial, zeroes) = parse_instruction_part2(dial, line);
        pointed_numbers.push(dial);
        rotated_zeroes += zeroes;
    }

    let zero_count = pointed_numbers.iter().filter(|&&x| x == 0).count();

    Some(zero_count as u64 + rotated_zeroes)
}

pub fn parse_instruction_part2(current_number: u64, instruction: &str) -> (u64, u64) {
    // Returns final number dial lands on, and the number of times the dial
    // goes past 0, in either direction
    let (direction, distance_str) = instruction.split_at(1);
    let distance: i32 = distance_str.parse().unwrap();

    // how to account for the case where we rotate right and land on 100 aka 0?
    // or rotate left and land on -100? what about 200 or -200?
    let mut rotated_zeroes: u64 = (distance / 100).abs() as u64;
    let effective_distance: i32 = distance % 100;

    let result: i32 = match direction {
        "L" => current_number as i32 - effective_distance,
        "R" => current_number as i32 + effective_distance,
        _ => 0,
    };

    // If we didn't start at 0 and we're negative, we've rotated past 0 at least once
    if (result < 0 || result > 100) && current_number != 0 {
        rotated_zeroes += 1;
    }

    let landed_number: u64 = (((result % 100) + 100) as u64) % 100 as u64;

    (landed_number, rotated_zeroes)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_02() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_03() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1));
    }
}
