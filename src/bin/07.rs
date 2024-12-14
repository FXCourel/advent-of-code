advent_of_code::solution!(7);

fn is_target_reachable(target: i64, numbers: &mut Vec<i64>, operator_pos: usize, allow_concatenate: bool) -> bool {
    if operator_pos == numbers.len() - 1 {
        if numbers[operator_pos] == target {
            return true;
        }
        return false;
    } else if numbers[operator_pos] > target {
        return false;
    }
    // Test with +
    let a = numbers[operator_pos];
    let b = numbers[operator_pos + 1];
    numbers[operator_pos + 1] = a + b;
    if is_target_reachable(target, numbers, operator_pos + 1, allow_concatenate) {
        return true;
    }
    // Test with *
    numbers[operator_pos + 1] = a * b;
    if is_target_reachable(target, numbers, operator_pos + 1, allow_concatenate) {
        return true;
    }
    if allow_concatenate {
        // Test with ||
        let str_a = a.to_string();
        let str_b = b.to_string();
        numbers[operator_pos + 1] = (str_a + &str_b).parse().unwrap();
        if is_target_reachable(target, numbers, operator_pos + 1, allow_concatenate) {
            return true;
        }
    }
    // Reset
    numbers[operator_pos + 1] = b;
    false
}

fn solve(input: &str, allow_concatenate: bool) -> Option<u64> {
    let input = advent_of_code::parser::to_tuples(input, ":", " ");
    let input = advent_of_code::parser::conversion::to_int_tuples_vec(input);
    let mut count = 0;
    for (target, mut numbers) in input {
        if is_target_reachable(target, &mut numbers, 0, allow_concatenate) {
            count += target;
        }
    }
    Some(count as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
