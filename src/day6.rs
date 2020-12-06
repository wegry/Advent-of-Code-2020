use std::collections::BTreeSet;
use std::fs;

pub const DAY: u16 = 6;

fn parse_str(raw: &str) -> Vec<BTreeSet<char>> {
    raw.split("\n\n")
        .into_iter()
        .map(|group| group.lines().flat_map(|form| form.chars()).collect())
        .collect()
}

fn parse_str_2(raw: &str) -> Vec<BTreeSet<char>> {
    raw.split("\n\n")
        .into_iter()
        .map(|group| {
            let group_members = group
                .lines()
                .map(|form| form.chars().collect::<BTreeSet<_>>())
                .collect::<Vec<_>>();
            let n = group_members.len();
            match n {
                0 => BTreeSet::new(),
                _ => {
                    let mut acc = group_members
                        .first()
                        .unwrap()
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<_>>();

                    match n {
                        1 => acc,
                        _ => {
                            let rest = &group_members[1..];

                            for curr in rest {
                                acc = acc.intersection(&curr).copied().collect();
                            }

                            acc
                        }
                    }
                }
            }
        })
        .collect()
}

fn count_set_cardinality<T>(s: Vec<BTreeSet<T>>) -> usize {
    s.into_iter().map(|group| group.len()).sum()
}

fn parse() -> Vec<BTreeSet<char>> {
    let raw = fs::read_to_string("./src/day6.txt").unwrap();
    parse_str(&raw)
}

fn parse_2() -> Vec<BTreeSet<char>> {
    let raw = fs::read_to_string("./src/day6.txt").unwrap();
    parse_str_2(&raw)
}

pub fn part_1() {
    let data = parse();
    let counts: usize = count_set_cardinality(data);

    println!(
        "There are {} distinct answers from the groups collectively",
        counts
    )
}

pub fn part_2() {
    let data = parse_2();
    let counts: usize = count_set_cardinality(data);

    println!(
        "There are {} unanimous distinct answers from the groups collectively",
        counts
    )
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const RAW_DATA: &str = indoc! {
        "abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b"
    };

    #[test]
    fn parse() {
        let data = super::parse_str(RAW_DATA);

        assert_eq!(data[1], btreeset!('a', 'b', 'c'));
    }

    #[test]
    fn parse_2() {
        let data = super::parse_str_2(RAW_DATA);
        let counts = super::count_set_cardinality(data);

        assert_eq!(counts, 6);
    }
}
