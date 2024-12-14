advent_of_code::solution!(5);
use advent_of_code::parser;
use std::collections::HashSet;

pub fn part_one(_input: &str) -> Option<u64> {
    let mut split_input = _input.trim().split("\n\n");
    let page_ordering_rules_section = split_input.next().unwrap().trim();
    let updates_section = split_input.next().unwrap().trim();

    let mut page_ordering_rules = HashSet::new();
    for line in page_ordering_rules_section.lines() {
        let mut split_line = line.split("|");
        let inf = split_line.next().unwrap();
        let sup = split_line.next().unwrap();
        page_ordering_rules.insert((inf, sup));
    }

    let updates = parser::to_matrix(updates_section, ",");
    let mut result = 0;
    'updates: for line in updates {
        for i in 0..line.len() {
            for j in i + 1..line.len() {
                if page_ordering_rules.contains(&(line[j], line[i])) {
                    continue 'updates;
                }
            }
        }
        result += line[(line.len() - 1) / 2].parse::<u64>().unwrap();
    }

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut split_input = _input.trim().split("\n\n");
    let page_ordering_rules_section = split_input.next().unwrap().trim();
    let updates_section = split_input.next().unwrap().trim();

    let mut page_ordering_rules = HashSet::new();
    for line in page_ordering_rules_section.lines() {
        let mut split_line = line.split("|");
        let inf = split_line.next().unwrap();
        let sup = split_line.next().unwrap();
        page_ordering_rules.insert((inf, sup));
    }

    let mut result = 0;
    let updates = parser::to_matrix(updates_section, ",");
    let compare = |a: &str, b: &str| {
        if page_ordering_rules.contains(&(a, b)) {
            std::cmp::Ordering::Less
        } else if page_ordering_rules.contains(&(b, a)) {
            std::cmp::Ordering::Greater
        } else {
            panic!("Invalid ordering rule");
        }
    };
    for line in updates.into_iter() {
        if line.is_sorted_by(|a, b| page_ordering_rules.contains(&(a, b))) {
            continue;
        }
        let mut line = line.to_owned();
        line.sort_by(|a, b| compare(a, b));
        result += line[(line.len() - 1) / 2].parse::<u64>().unwrap();
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
