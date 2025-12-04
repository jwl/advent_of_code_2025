advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltages: Vec<u64> = Vec::new();

    for line in input.lines() {
        let bank = line.trim();
        joltages.push(get_max_joltage_part1(bank));
    }

    Some(joltages.iter().sum())
}

fn get_max_joltage_part1(bank: &str) -> u64 {
    // println!("Looking for max joltage in bank {bank}");

    let trimmed = &bank[..bank.len() - 1];

    let (first_battery_idx, first_battery_joltage) = get_max_battery(trimmed);

    // Now starting from first_battery_idx, find the largest battery
    // to the right
    let second_trimmed = &bank[first_battery_idx + 1..];
    let (_second_battery_idx, second_battery_joltage) = get_max_battery(second_trimmed);

    format!("{first_battery_joltage}{second_battery_joltage}")
        .parse()
        .unwrap()
}

// Returns index and joltage of max battery in bank.
// In case of ties, returns LEFT-MOST battery
fn get_max_battery(bank: &str) -> (usize, u64) {
    let mut max_idx = 0;
    let mut max_joltage = 0;

    for (i, c) in bank.char_indices() {
        let joltage = c.to_digit(10).expect("Should be a digit");
        if joltage > max_joltage {
            max_joltage = joltage;
            max_idx = i;
        }
    }
    (max_idx, max_joltage as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltages: Vec<u64> = Vec::new();

    for line in input.lines() {
        let bank = line.trim();
        joltages.push(get_max_joltage_part2(bank));
    }

    Some(joltages.iter().sum())
}

fn get_max_joltage_part2(bank: &str) -> u64 {
    let number_of_batteries = 12;

    // (index_in_bank, joltage)
    let mut batteries: Vec<(usize, u64)> = Vec::new();

    // for first battery, find the largest digit from the start of the bank
    // to bank.len()-number_of_batteries, so you can still make
    // a full 12 battery joltage
    let mut left_most_index = 0;
    for battery in 1..(number_of_batteries + 1) {
        let trimmed_bank = &bank[left_most_index..bank.len() - (number_of_batteries - battery)];
        let (mut max_battery_idx, max_battery_joltage) = get_max_battery(trimmed_bank);
        max_battery_idx += left_most_index;
        left_most_index = max_battery_idx + 1;
        batteries.push((max_battery_idx, max_battery_joltage));
    }

    // Concatenate every battery's joltage into a giant number and return as u64
    batteries
        .iter()
        .map(|&(_, joltage)| joltage.to_string())
        .collect::<String>()
        .parse()
        .expect("Concatenated joltage was not a valid u64")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_one_02() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(98));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
