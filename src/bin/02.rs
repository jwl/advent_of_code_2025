advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    // Split input by commas to get each range
    let ranges: Vec<&str> = input.trim().split(',').collect();

    // For each range, find all invalid IDs
    let mut all_invalid_ids: Vec<u64> = Vec::new();
    for range in &ranges {
        let mut invalid_ids: Vec<u64> = get_invalid_ids(range).ok()?;
        // println!("For range {range}, we found {}", invalid_ids.len());
        all_invalid_ids.append(&mut invalid_ids);
    }

    // Sum up all invalid IDs
    Some(all_invalid_ids.iter().sum())
}

fn get_invalid_ids(range: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    // println!("Trying to find all invalid IDs within range {range}");
    let (lower_bound, upper_bound) = parse_range(range)?;
    // println!("lower_bound is {lower_bound}, and upper_bound is {upper_bound}");

    let mut invalid_ids: Vec<u64> = Vec::new();

    for i in lower_bound..=upper_bound {
        if is_invalid_id(i) {
            // println!("Found an invalid id: {i}");
            invalid_ids.push(i);
        }
    }

    Ok(invalid_ids)
}

fn parse_range(range: &str) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let mut parts = range.split('-');
    let lower_bound: u64 = parts.next().ok_or("Missing lower_bound")?.parse()?;
    let upper_bound: u64 = parts.next().ok_or("Missing upper_bound")?.parse()?;

    Ok((lower_bound, upper_bound))
}

fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    if s.len() % 2 != 0 {
        return false;
    }

    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);

    if left == right {
        return true;
    } else {
        return false;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    // Split input by commas to get each range
    let ranges: Vec<&str> = input.trim().split(',').collect();

    // For each range, find all invalid IDs
    let mut all_invalid_ids: Vec<u64> = Vec::new();
    for range in &ranges {
        let mut invalid_ids: Vec<u64> = get_invalid_ids_part2(range).ok()?;
        // println!("For range {range}, we found {}", invalid_ids.len());
        all_invalid_ids.append(&mut invalid_ids);
    }

    // Sum up all invalid IDs
    Some(all_invalid_ids.iter().sum())
}

fn get_invalid_ids_part2(range: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    // println!("Trying to find all invalid IDs within range {range}");
    let (lower_bound, upper_bound) = parse_range(range)?;
    // println!("lower_bound is {lower_bound}, and upper_bound is {upper_bound}");

    let mut invalid_ids: Vec<u64> = Vec::new();

    for i in lower_bound..=upper_bound {
        if is_invalid_id_part2(i) {
            // println!("Found an invalid id: {i}");
            invalid_ids.push(i);
        }
    }

    Ok(invalid_ids)
}

fn is_invalid_id_part2(id: u64) -> bool {
    // A string consists of the same pattern repeated multiple times
    // if and only if the string is a nontrivial rotation of itself.
    let s = id.to_string();
    let mut t = s.clone() + &s;

    // Take the first and last characters off to prevent the trival rotation
    t = t[1..t.len() - 1].to_string();

    // ie. 5656 is a substring of 656565
    t.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_one_01() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10516506));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
