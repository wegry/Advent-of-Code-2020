use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub const DAY: u16 = 3;

#[derive(Clone, Debug, PartialEq)]
enum Place {
    Tree,
    Open,
}

type Matrix = Vec<Vec<Place>>;

impl FromStr for Place {
    type Err = ParseIntError;
    fn from_str(letter: &str) -> Result<Self, Self::Err> {
        match letter {
            "." => Ok(Place::Open),
            "#" => Ok(Place::Tree),
            x => panic!("[{:?}] isn't a valid character...", x),
        }
    }
}

fn parse_str(s: &str) -> Matrix {
    let text = s.lines();

    text.into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter_map(|letter| {
                    if letter == ' ' {
                        None
                    } else {
                        Some(letter.to_string().parse::<Place>().unwrap())
                    }
                })
                .collect()
        })
        .collect()
}

fn parse() -> Matrix {
    let raw = fs::read_to_string("./src/day3.txt").unwrap();
    parse_str(&raw)
}

fn count_trees_with_slope(m: Matrix, (right, down): (usize, usize)) -> usize {
    let mut tree_count = 0;
    let mut x = 0;
    let mut y = 0;

    let m_height = m.len();
    let m_width = m[0].len();

    while y < m_height {
        if m[y][x] == Place::Tree {
            tree_count += 1;
        }

        x = (x + right) % m_width;
        y += down;
    }

    tree_count
}

fn count_trees(m: Matrix) -> usize {
    count_trees_with_slope(m, (3, 1))
}

pub fn part_1() {
    let data = parse();
    let tree_count = count_trees(data);

    println!("Trees encountered: {:?}", tree_count)
}

// (right, down)
const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub fn part_2() {
    let data = parse();

    let result: usize = SLOPES
        .iter()
        .map(|s| count_trees_with_slope(data.clone(), *s))
        .product();

    println!("The product of tree counts at slopes: {}", result);
}

#[cfg(test)]
mod tests {

    const RAW_DATA: &str = "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

    #[test]
    fn parse() {
        let data = super::parse_str(RAW_DATA);

        assert_eq!(super::Place::Tree, data[0][2]);
    }

    #[test]
    fn count_trees() {
        let data = super::parse_str(RAW_DATA);

        assert_eq!(super::count_trees(data), 7);
    }

    #[test]
    fn count_trees_with_slope() {
        let data = super::parse_str(RAW_DATA);

        let result = super::SLOPES
            .iter()
            .map(|s| super::count_trees_with_slope(data.clone(), *s))
            .collect::<Vec<_>>();

        assert_eq!([2, 7, 3, 4, 2], result[..]);
    }
}
