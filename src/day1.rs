use std::fs;

pub const DAY: u16 = 1;

fn parse_str(raw: &str) -> Vec<u64> {
    let text = raw.split_ascii_whitespace();

    text.into_iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

fn parse() -> Vec<u64> {
    let raw = fs::read_to_string("./src/day1.txt").unwrap();
    parse_str(&raw)
}

fn find_pair(candidates: Vec<u64>) -> Option<(u64, u64)> {
    for i in &candidates {
        for j in &candidates {
            if i + j == 2020 {
                return Some((*i, *j));
            }
        }
    }

    None
}

fn find_triple(candidates: Vec<u64>) -> Option<(u64, u64, u64)> {
    for i in &candidates {
        for j in &candidates {
            for k in &candidates {
                if i + j + k == 2020 {
                    return Some((*i, *j, *k));
                }
            }
        }
    }

    None
}

pub fn part_1() {
    let data = parse();
    let pair = find_pair(data).unwrap();
    let (i, j) = pair;
    println!("{:?} multiplied together has the product {}", pair, i * j)
}

pub fn part_2() {
    let data = parse();
    let triple = find_triple(data).unwrap();
    let (i, j, k) = triple;

    println!(
        "{:?} multiplied together has the product {}",
        triple,
        i * j * k
    )
}

#[cfg(test)]
mod tests {
    const RAW_DATA: &str = "1721
        979
        366
        299
        675
        1456";

    #[test]
    fn find_pair_works() {
        let data = super::parse_str(RAW_DATA);
        let res = super::find_pair(data);
        assert_eq!(Some((1721, 299)), res);
    }

    #[test]
    fn find_triple_works() {
        let data = super::parse_str(RAW_DATA);
        let res = super::find_triple(data);
        assert_eq!(Some((979, 366, 675)), res);
    }
}
