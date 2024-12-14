use std::collections::HashMap;

pub fn to_single_line(input: &str) -> &str {
    input.trim().lines().next().unwrap().trim()
}

pub fn to_column(input: &str) -> Vec<&str> {
    input.trim().lines().map(|line| line.trim()).collect()
}

pub fn to_map(input: &str) -> super::map::Map {
    super::map::Map::new(input)
}

pub fn to_matrix<'a>(input: &'a str, split_pattern: &'a str) -> Vec<Vec<&'a str>> {
    input
        .trim()
        .lines()
        .map(|line| line.split(split_pattern).collect())
        .collect()
}

pub fn to_dict<'a>(
    input: &'a str,
    split_pattern_key: &'a str,
    split_pattern_value: &'a str,
) -> HashMap<&'a str, Vec<&'a str>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, split_pattern_key);
            let key = parts.next().unwrap().trim();
            let value = parts
                .next()
                .unwrap()
                .trim()
                .split(split_pattern_value)
                .collect();
            (key, value)
        })
        .collect()
}

pub fn to_tuples<'a>(input: &'a str, split_pattern_key: &'a str, split_pattern_value: &'a str) -> Vec<(&'a str, Vec<&'a str>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, split_pattern_key);
            let key = parts.next().unwrap().trim();
            let value = parts
                .next()
                .unwrap()
                .trim()
                .split(split_pattern_value)
                .collect();
            (key, value)
        })
        .collect()
}


pub mod conversion {

    use std::collections::HashMap;

    pub fn to_int_vec(vec: Vec<&str>) -> Vec<i64> {
        vec.iter().map(|&x| x.parse().unwrap()).collect()
    }

    pub fn to_int_matrix(matrix: Vec<Vec<&str>>) -> Vec<Vec<i64>> {
        matrix
            .iter()
            .map(|row| row.iter().map(|&x| x.parse().unwrap()).collect())
            .collect()
    }

    pub fn to_int_dict<'a>(dict: HashMap<&'a str, &'a str>) -> HashMap<i64, i64> {
        dict.iter()
            .map(|(key, value)| {
                let key = key.parse().unwrap();
                let value = value.parse().unwrap();
                (key, value)
            })
            .collect()
    }

    pub fn to_int_dict_vec<'a>(dict: HashMap<&'a str, Vec<&'a str>>) -> HashMap<i64, Vec<i64>> {
        dict.iter()
            .map(|(key, value)| {
                println!("{:?} {:?}", key, value);
                let key = key.parse().unwrap();
                let value = value.iter().map(|&x| x.parse().unwrap()).collect();
                (key, value)
            })
            .collect()
    }

    pub fn to_int_tuples_vec<'a>(tuples: Vec<(&'a str, Vec<&'a str>)>) -> Vec<(i64, Vec<i64>)> {
        tuples.iter()
            .map(|(key, value)| {
                let key = key.parse().unwrap();
                let value = value.iter().map(|&x| x.parse().unwrap()).collect();
                (key, value)
            })
            .collect()
    }
}

#[cfg(feature = "test_aoc_lib")]
mod tests {

    #[test]
    fn test_to_single_line() {
        let input = " Hello\t my \nWorld";
        let result = super::to_single_line(input);
        assert_eq!(result, "Hello\t my");
    }

    #[test]
    fn test_to_column() {
        let input = " Hello\t my \nWorld";
        let result = super::to_column(input);
        assert_eq!(result, vec!["Hello\t my", "World"]);
    }

    #[test]
    fn test_to_map() {
        let input = " X..X.\n..X..\n\n\n";
        let result = super::to_map(input);
        assert_eq!(
            result.grid,
            vec![vec!['X', '.', '.', 'X', '.'], vec!['.', '.', 'X', '.', '.']]
        );
        assert_eq!(result.size(), (5, 2));
    }

    #[test]
    fn test_to_matrix() {
        let input = "1,2,3\n4,5,6\n7,8,9";
        let result = super::to_matrix(input, ",");
        assert_eq!(
            result,
            vec![
                vec!["1", "2", "3"],
                vec!["4", "5", "6"],
                vec!["7", "8", "9"]
            ]
        );
    }

    #[test]
    fn test_to_dict() {
        let input = "key1: 10 11 12 \nkey2: 21 22 23\nkey3: 31 32 \n\n";
        let result = super::to_dict(input, ":", " ");
        let mut expected = std::collections::HashMap::new();
        expected.insert("key1", vec!["10", "11", "12"]);
        expected.insert("key2", vec!["21", "22", "23"]);
        expected.insert("key3", vec!["31", "32"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_conversion_to_int_vec() {
        let input = vec!["1", "2", "3"];
        let result = super::conversion::to_int_vec(input);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_conversion_to_int_matrix() {
        let input = vec![
            vec!["1", "2", "3"],
            vec!["4", "5", "6"],
            vec!["7", "8", "9"],
        ];
        let result = super::conversion::to_int_matrix(input);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn test_conversion_to_int_dict() {
        let mut input = std::collections::HashMap::new();
        input.insert("1", "2");
        input.insert("3", "4");
        let result = super::conversion::to_int_dict(input);
        let mut expected = std::collections::HashMap::new();
        expected.insert(1, 2);
        expected.insert(3, 4);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_conversion_to_int_dict_vec() {
        let mut input = std::collections::HashMap::new();
        input.insert("1", vec!["2", "3"]);
        input.insert("4", vec!["5", "6"]);
        let result = super::conversion::to_int_dict_vec(input);
        let mut expected = std::collections::HashMap::new();
        expected.insert(1, vec![2, 3]);
        expected.insert(4, vec![5, 6]);
        assert_eq!(result, expected);
    }
}
