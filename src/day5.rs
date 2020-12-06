use std::cmp::Ordering;
use std::convert::Infallible;
use std::fs;
use std::str::FromStr;

pub const DAY: u16 = 5;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        (self.row * 8) + self.col
    }
}

impl FromStr for Seat {
    type Err = Infallible;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut col = 0;
        let mut row = 0;

        let as_chars = raw.chars().collect::<Vec<_>>();

        for i in 0..7 {
            match as_chars[i] {
                'F' => (),
                'B' => row |= 1 << (6 - i),
                _ => (),
            }
        }

        for i in 0..3 {
            match as_chars[i + 7 /* ignore row designators */] {
                'R' => col |= 1 << (2 - i),
                'L' => (),
                _ => (),
            }
        }

        Ok(Seat { row, col })
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

fn parse() -> Vec<Seat> {
    let raw = fs::read_to_string("./src/day5.txt").unwrap();
    raw.lines().map(|l| l.parse::<Seat>().unwrap()).collect()
}

pub fn part_1() {
    let data = parse();

    let max_id = data.into_iter().max_by(|x, y| x.id().cmp(&y.id())).unwrap();
    println!("The max seat id is {}", max_id.id());
}

pub fn part_2() {
    let mut raw = parse();
    raw.sort();
    let data = &raw[1..raw.len() - 1];
    let your_seat = (*data)
        .windows(2)
        .find(|slice| match slice {
            [x, y] => x.id() + 2 == y.id(),
            _ => false,
        })
        .map(|slice| match slice {
            [x, _] => Some(x.id() + 1),
            _ => None,
        })
        .flatten()
        .unwrap();
    println!("Your seat id is {}", your_seat)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let data = "BFFFBBFRRR".parse::<super::Seat>().unwrap();

        assert_eq!(data, super::Seat { row: 70, col: 7 });
        assert_eq!(data.id(), 567);
    }
}
