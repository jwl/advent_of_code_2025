advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltages: Vec<u64> = Vec::new();

    for line in input.lines() {
        let bank = line.trim();
        joltages.push(get_max_joltage(bank));
    }

    println!("joltages: {:?}", joltages);

    Some(joltages.iter().sum())
}

fn get_max_joltage(bank: &str) -> u64 {
    // println!("Looking for max joltage in bank {bank}");

    let trimmed = &bank[..bank.len() - 1];

    let (first_battery_idx, first_battery_joltage) = get_max_battery(trimmed);
    // println!(
    //     "\tFirst battery is of value {first_battery_joltage} and is at index {first_battery_idx}"
    // );

    // Now starting from first_battery_idx, find the largest battery
    // to the right
    let second_trimmed = &bank[first_battery_idx + 1..];
    let (second_battery_idx, second_battery_joltage) = get_max_battery(second_trimmed);

    // println!(
    //     "\tSecond battery is of value {second_battery_joltage} and is at index {:?}",
    //     second_battery_idx + first_battery_idx
    // );

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
    None
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
        assert_eq!(result, None);
    }
}
