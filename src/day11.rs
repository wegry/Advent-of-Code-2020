use itertools::{join, Itertools};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::convert::Infallible;
use std::fs;
use std::str::FromStr;

pub const DAY: u16 = 11;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SeatingSystem {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl FromStr for SeatingSystem {
    type Err = Infallible;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(match raw {
            "L" => SeatingSystem::EmptySeat,
            "." => SeatingSystem::Floor,
            "#" => SeatingSystem::OccupiedSeat,
            x => panic!("{:?} isn't a valid string.", x),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Spot {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    RightAndUp,
    Right,
    RightAndDown,
    Down,
    LeftAndDown,
    Left,
    LeftAndUp,
}

impl Spot {
    fn look(&self, direction: Dir) -> Option<Spot> {
        lazy_static! {
            static ref TOP_EDGE: HashSet<Dir> = hashset! {Dir::Up, Dir::RightAndUp, Dir::LeftAndUp};
            static ref LEFT_EDGE: HashSet<Dir> =
                hashset! {Dir::LeftAndUp, Dir::Left, Dir::LeftAndDown};
        };
        let Spot {
            row: row_ref,
            col: col_ref,
        } = &self;
        let row = *row_ref;
        let col = *col_ref;
        // Don't fall off the edge of usize
        if (row == 0 && TOP_EDGE.contains(&direction))
            || (col == 0 && LEFT_EDGE.contains(&direction))
        {
            return None;
        }

        let (next_col, next_row) = match direction {
            Dir::Up => (col, row - 1),
            Dir::RightAndUp => (col + 1, row - 1),
            Dir::Right => (col + 1, row),
            Dir::RightAndDown => (col + 1, row + 1),
            Dir::Down => (col, row + 1),
            Dir::LeftAndDown => (col - 1, row + 1),
            Dir::Left => (col - 1, row),
            Dir::LeftAndUp => (col - 1, row - 1),
        };

        Some(Spot {
            col: next_col,
            row: next_row,
        })
    }
}

type Grid = BTreeMap<Spot, SeatingSystem>;

#[allow(dead_code)]
fn as_string(grid: &Grid) -> String {
    let mut grouped: BTreeMap<usize, BTreeSet<Spot>> = btreemap! {};
    for (key, group) in &grid.keys().group_by(|Spot { row, .. }| *row) {
        let as_set = group.copied().collect::<BTreeSet<_>>();
        grouped
            .entry(key)
            .and_modify(|v| *v = v.union(&as_set).copied().collect())
            .or_insert_with(|| as_set);
    }

    join(
        grouped.values().map(|points| {
            join(
                points.iter().map(|p| match grid[p] {
                    SeatingSystem::EmptySeat => "L",
                    SeatingSystem::Floor => ".",
                    SeatingSystem::OccupiedSeat => "#",
                }),
                "",
            )
        }),
        "\n",
    )
}

fn model_arrivals(original: Grid) -> Grid {
    let grid = &original;
    let mut next = grid.clone();
    let search_range = &(0..=2);

    for (Spot { col: i, row: j }, &kind) in grid
        .iter()
        .filter(|(_, &seat)| seat != SeatingSystem::Floor)
    {
        let neighboring_occupied: usize = search_range
            .clone()
            .map(move |x| {
                search_range
                    .clone()
                    .filter_map(move |y| {
                        if (*i == 0 && x == 0) || (*j == 0 && y == 0) || (x == y && x == 1) {
                            None
                        } else {
                            // 0..=2 means subtract 1 here.
                            let i_2 = i + x - 1;
                            let j_2 = j + y - 1;

                            if grid.get(&Spot { col: i_2, row: j_2 })
                                == Some(&SeatingSystem::OccupiedSeat)
                            {
                                return Some(1);
                            }

                            None
                        }
                    })
                    .count()
            })
            .sum();
        if kind == SeatingSystem::EmptySeat && neighboring_occupied == 0 {
            next.insert(Spot { col: *i, row: *j }, SeatingSystem::OccupiedSeat);
        } else if kind == SeatingSystem::OccupiedSeat && neighboring_occupied >= 4 {
            next.insert(Spot { col: *i, row: *j }, SeatingSystem::EmptySeat);
        }
    }

    next
}

fn search_in_dir_from(spot: Spot, grid: &Grid, direction: Dir) -> Option<Spot> {
    lazy_static! {
        static ref TOP_EDGE: HashSet<Dir> = hashset! {Dir::Up, Dir::RightAndUp, Dir::LeftAndUp};
        static ref LEFT_EDGE: HashSet<Dir> = hashset! {Dir::LeftAndUp, Dir::Left, Dir::LeftAndDown};
    };

    let maybe_spot = spot.look(direction);

    let next_spot = maybe_spot?;

    let next_entry = grid.get(&next_spot);
    next_entry
        .map(|spot| match &spot {
            SeatingSystem::EmptySeat => None,
            SeatingSystem::Floor => search_in_dir_from(next_spot, grid, direction),
            SeatingSystem::OccupiedSeat => Some(next_spot),
        })
        .flatten()
}

lazy_static! {
    static ref DIRECTIONS: [Dir; 8] = [
        Dir::Up,
        Dir::RightAndUp,
        Dir::Right,
        Dir::RightAndDown,
        Dir::Down,
        Dir::LeftAndDown,
        Dir::Left,
        Dir::LeftAndUp,
    ];
}

fn count_visibly_occupied(spot: Spot, grid: &Grid) -> usize {
    DIRECTIONS
        .iter()
        .filter_map(|direction| search_in_dir_from(spot, grid, *direction))
        .count()
}

fn model_arrivals_2(original: Grid) -> Grid {
    let grid = &original;
    let mut next = grid.clone();

    for (spot, &kind) in grid.iter() {
        let visibly_occupied = count_visibly_occupied(*spot, grid);
        if kind == SeatingSystem::EmptySeat && visibly_occupied == 0 {
            next.insert(*spot, SeatingSystem::OccupiedSeat);
        } else if kind == SeatingSystem::OccupiedSeat && visibly_occupied >= 5 {
            next.insert(*spot, SeatingSystem::EmptySeat);
        }
    }

    next
}

fn parse_str(raw: &str) -> Grid {
    raw.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                (
                    Spot { row: y, col: x },
                    c.to_string().parse::<SeatingSystem>().unwrap(),
                )
            })
        })
        .collect()
}

fn iter_and_check(grid: Grid, f: &dyn Fn(Grid) -> Grid) -> usize {
    let mut prev = grid.clone();
    let mut curr = grid;

    loop {
        curr = f(curr.clone());
        if prev == curr {
            break;
        }

        prev = curr.clone()
    }

    let occupied_count = curr
        .values()
        .copied()
        .filter(|&c| c == SeatingSystem::OccupiedSeat)
        .count();

    dbg!(occupied_count);
    occupied_count
}

pub fn part_1() {
    let raw = fs::read_to_string("./src/day11.txt").unwrap();
    let grid = parse_str(&raw);

    let occupied_count = iter_and_check(grid, &model_arrivals);

    println!("{:?} seats were occupied", occupied_count)
}

pub fn part_2() {
    let raw = fs::read_to_string("./src/day11.txt").unwrap();
    let grid = parse_str(&raw);

    let occupied_count = iter_and_check(grid, &model_arrivals_2);

    println!("{:?} seats were occupied", occupied_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const RAW_DATA: &str = indoc!(
        "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL"
    );

    #[test]
    fn parse() {
        let parsed = parse_str(RAW_DATA);
        assert_eq!(parsed[&Spot { row: 0, col: 0 }], SeatingSystem::EmptySeat);
        assert_eq!(parsed[&Spot { row: 0, col: 1 }], SeatingSystem::Floor)
    }

    #[test]
    fn model_arrivals_() {
        let parsed = parse_str(RAW_DATA);
        let one_iteration = model_arrivals(parsed);
        let two_iterations = model_arrivals(one_iteration.clone());
        let three_iterations = model_arrivals(two_iterations.clone());
        assert_eq!(
            one_iteration[&Spot { row: 0, col: 0 }],
            SeatingSystem::OccupiedSeat
        );
        assert_eq!(
            two_iterations[&Spot { row: 0, col: 0 }],
            SeatingSystem::OccupiedSeat
        );
        assert_eq!(
            three_iterations[&Spot { row: 0, col: 0 }],
            SeatingSystem::OccupiedSeat
        )
    }

    #[test]
    fn first_visible_seat() {
        let input = indoc!(
            ".............
        .L.L.#.#.#.#.
        ............."
        );
        let grid = parse_str(input);
        let should_not_find_visible = search_in_dir_from(Spot { row: 1, col: 0 }, &grid, Dir::Down);
        let should_not_find_visible_2 =
            search_in_dir_from(Spot { row: 2, col: 12 }, &grid, Dir::LeftAndUp);
        let should_find_visible = search_in_dir_from(Spot { row: 1, col: 1 }, &grid, Dir::Right);
        let should_find_visible_2 = search_in_dir_from(Spot { row: 1, col: 3 }, &grid, Dir::Right);

        println!("{}", as_string(&grid));

        assert_eq!(should_not_find_visible, None);
        assert_eq!(should_not_find_visible_2, None);
        assert_eq!(should_find_visible, Some(Spot { col: 5, row: 1 }));
        assert_eq!(should_find_visible_2, Some(Spot { col: 5, row: 1 }))
    }

    #[test]
    fn part_2_11() {
        let mut parsed = parse_str(RAW_DATA);
        let stages = vec![
            indoc!(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            ),
            indoc!(
                "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            ),
            indoc!(
                "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#"
            ),
            indoc!(
                "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#"
            ),
            indoc!(
                "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#"
            ),
            indoc!(
                "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
            ),
            indoc!(
                "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
            ),
        ];

        for (_, stage) in stages.iter().copied().enumerate() {
            assert_eq!(as_string(&parsed), stage);

            parsed = model_arrivals_2(parsed);
        }
    }
}
