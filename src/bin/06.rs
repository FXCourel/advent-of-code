advent_of_code::solution!(6);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let map = advent_of_code::parser::to_map(input);
    let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();

    let (mut x, mut y) = map.find('^').unwrap();

    // Walk the path
    let mut places = HashSet::new();
    let (mut dx, mut dy) = directions.next().unwrap();
    loop {
        places.insert((x, y));
        let attempt = (x + dx, y + dy);
        if !(map.is_in_bounds(attempt.0, attempt.1)) {
            break;
        } else if map.get(attempt.0, attempt.1) == '#' {
            (dx, dy) = *directions.next().unwrap();
        } else {
            (x, y) = attempt;
        }
    }

    Some(places.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = advent_of_code::parser::to_map(input);

    let (x_start, y_start) = map.find('^').unwrap();

    let mut result = 0;
    for i in 0..map.height {
        for j in 0..map.width {
            let (mut x, mut y) = (x_start, y_start);
            let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
            let mut places = HashSet::new();
            let (mut dx, mut dy) = directions.next().unwrap();
            result += loop {
                if places.contains(&(x, y, dx, dy)) {
                    break 1;
                }
                places.insert((x, y, dx, dy));
                let attempt = (x + dx, y + dy);
                if !(map.is_in_bounds(attempt.0, attempt.1)) {
                    break 0;
                } else if map.get(attempt.0, attempt.1) == '#' || attempt == (j as isize, i as isize) {
                    (dx, dy) = *directions.next().unwrap();
                } else {
                    (x, y) = attempt;
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
