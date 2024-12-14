#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone)]
pub struct Map {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        0 <= y && y < self.height as isize && 0 <= x && x < self.width as isize
    }

    pub fn get(&self, x: isize, y: isize) -> char {
        assert!(self.is_in_bounds(x, y));
        self.grid[y as usize][x as usize]
    }

    pub fn set(&mut self, x: isize, y: isize, value: char) {
        assert!(self.is_in_bounds(x, y));
        self.grid[y as usize][x as usize] = value;
    }

    pub fn size(&self) -> (isize, isize) {
        (self.width as isize, self.height as isize)
    }

    pub fn find(&self, value: char) -> Option<(isize, isize)> {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|v| *v == value).map(|x| (x as isize, y as isize)))
    }

    pub fn find_all(&self, value: char) -> Vec<(isize, isize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(
                    move |(x, v)| {
                        if *v == value {
                            Some((x as isize, y as isize))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_left(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn half_turn(&mut self) {
        *self = match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn move_forward(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Self::Up => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Up => "\u{2191}",
            Self::Right => "\u{2192}",
            Self::Down => "\u{2193}",
            Self::Left => "\u{2190}",
        };
        write!(f, "{}", string)
    }
}

#[cfg(feature = "test_aoc_lib")]
mod tests {
    #[test]
    fn test_map() {
        let input = "X..X.\n..X..\n\n\n";
        let mut map = super::Map::new(input);
        map.set(2, 0, 'O');
        assert_eq!(map.get(0, 0), 'X');
        assert_eq!(map.get(4, 0), '.');
        assert_eq!(map.get(0, 1), '.');
        assert_eq!(map.get(4, 1), '.');
        assert_eq!(map.get(2, 0), 'O');
        assert_eq!(map.size(), (5, 2));
        assert!(map.is_in_bounds(3, 1));
        assert!(!map.is_in_bounds(5, 2));
    }

    #[test]
    fn test_direction() {
        let mut direction = super::Direction::Up;
        assert_eq!(format!("{}", direction), "\u{2191}");
        direction.turn_left();
        assert_eq!(format!("{}", direction), "\u{2190}");
        direction.turn_right();
        assert_eq!(format!("{}", direction), "\u{2191}");
        direction.half_turn();
        assert_eq!(format!("{}", direction), "\u{2193}");
        assert_eq!(direction.move_forward(1, 2), (1, 3));
    }
}
