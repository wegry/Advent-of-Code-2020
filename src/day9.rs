use std::collections::BTreeSet;
use std::fs;

pub const DAY: u16 = 9;

fn parse_str(raw: &str) -> Vec<u128> {
    raw.lines().map(|l| l.parse::<u128>().unwrap()).collect()
}

fn first_not_sum_of_preable(preable: usize, candidates: Vec<u128>) -> Option<u128> {
    for window in candidates.windows(preable + 1) {
        let preable_nums = window[..=preable].iter().copied().collect::<BTreeSet<_>>();
        let current = window[preable];

        let is_sum = preable_nums.iter().any(|i| {
            preable_nums.iter().any(|j| {
                if i == j {
                    return false;
                }

                i + j == current
            })
        });

        if !is_sum {
            return Some(current);
        }
    }

    None
}

pub fn part_1() {
    let raw = fs::read_to_string("./src/day9.txt").unwrap();
    let parsed = parse_str(&raw);
    let result = first_not_sum_of_preable(25, parsed);

    dbg!(result);
}

fn find_continuous_number_set(matching: u128, candidates: Vec<u128>) -> Option<(u128, u128)> {
    for preable in 2..=candidates.len() {
        for window in candidates.windows(preable) {
            let preable_nums = window.iter().copied().collect::<BTreeSet<_>>();

            if preable_nums.iter().sum::<u128>() == matching {
                return Some((
                    *preable_nums.iter().min().unwrap(),
                    *preable_nums.iter().max().unwrap(),
                ));
            }
        }
    }

    None
}

pub fn part_2() {
    let raw = fs::read_to_string("./src/day9.txt").unwrap();
    let parsed = parse_str(&raw);

    let (fst, snd) = find_continuous_number_set(88311122, parsed).unwrap();

    dbg!(fst + snd);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const RAW_DATA: &str = indoc!(
        "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576"
    );
    #[test]
    fn first_not_sum_of_preable() {
        let parsed = super::parse_str(RAW_DATA);
        let result = super::first_not_sum_of_preable(5, parsed);

        assert_eq!(result, Some(127))
    }

    #[test]
    fn find_continuous_number_set() {
        let parsed = super::parse_str(RAW_DATA);
        let result = super::find_continuous_number_set(127, parsed);

        assert_eq!(result, Some((15, 47)))
    }
}
