use std::collections::BTreeMap;
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

type Grid = BTreeMap<(usize, usize), SeatingSystem>;

fn model_arrivals(original: Grid) -> Grid {
    let grid = &original;
    let mut next = grid.clone();
    let search_range = &(0..=2);

    for ((i, j), &kind) in grid
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

                            if grid.get(&(i_2, j_2)) == Some(&SeatingSystem::OccupiedSeat) {
                                return Some(1);
                            }

                            None
                        }
                    })
                    .count()
            })
            .sum();
        if kind == SeatingSystem::EmptySeat && neighboring_occupied == 0 {
            next.insert((*i, *j), SeatingSystem::OccupiedSeat);
        } else if kind == SeatingSystem::OccupiedSeat && neighboring_occupied >= 4 {
            next.insert((*i, *j), SeatingSystem::EmptySeat);
        }
    }

    next
}

fn parse_str(raw: &str) -> Grid {
    raw.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c.to_string().parse::<SeatingSystem>().unwrap()))
        })
        .collect()
}

pub fn part_1() {
    let raw = fs::read_to_string("./src/day11.txt").unwrap();
    let grid = parse_str(&raw);
    let mut prev = grid.clone();
    let mut curr = grid;

    loop {
        dbg!(curr
            .values()
            .copied()
            .filter(|&c| c == SeatingSystem::OccupiedSeat)
            .count());
        curr = model_arrivals(curr.clone());
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

    println!("{:?} seats were occupied", occupied_count)
}

pub fn part_2() {
    let raw = fs::read_to_string("./src/day11.txt").unwrap();
    let _program = parse_str(&raw);
}

#[cfg(test)]
mod tests {
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
        let parsed = super::parse_str(RAW_DATA);
        assert_eq!(parsed[&(0, 0)], super::SeatingSystem::EmptySeat);
        assert_eq!(parsed[&(0, 1)], super::SeatingSystem::Floor)
    }

    #[test]
    fn model_arrivals() {
        let parsed = super::parse_str(RAW_DATA);
        let one_iteration = super::model_arrivals(parsed);
        let two_iterations = super::model_arrivals(one_iteration.clone());
        let three_iterations = super::model_arrivals(two_iterations.clone());
        assert_eq!(one_iteration[&(0, 0)], super::SeatingSystem::OccupiedSeat);
        assert_eq!(two_iterations[&(0, 0)], super::SeatingSystem::OccupiedSeat);
        assert_eq!(
            three_iterations[&(0, 0)],
            super::SeatingSystem::OccupiedSeat
        )
    }

    #[test]
    fn part_2() {
        let parsed = super::parse_str(RAW_DATA);
    }
}
