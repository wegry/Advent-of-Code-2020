use std::fs;

pub const DAY: u16 = 10;

fn parse_str(raw: &str) -> Vec<u32> {
    let mut joltages = raw
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    joltages.push(0);
    joltages.push(joltages.iter().max().unwrap() + 3);
    joltages.sort();
    joltages
}

fn one_and_three_jolt_differences(joltages: Vec<u32>) -> (u32, u32) {
    joltages
        .windows(2)
        .fold((0, 0), |(one_jump, three_jump), curr| match curr {
            [prev, on] => {
                if on - prev == 1 {
                    (one_jump + 1, three_jump)
                } else if on - prev == 3 {
                    (one_jump, three_jump + 1)
                } else {
                    (one_jump, three_jump)
                }
            }
            x => panic!("{:?} shouldn't happen...", x),
        })
}

pub fn part_1() {
    let raw = fs::read_to_string("./src/day10.txt").unwrap();
    let parsed = parse_str(&raw);
    let (diff_1, diff_3) = one_and_three_jolt_differences(parsed);

    dbg!(diff_1, diff_3, diff_1 * diff_3);
}

fn count_possible_chains(from: Vec<u32>) -> u128 {
    let diff_chain = from
        .windows(2)
        .filter_map(|window| match &window {
            [x, y] if y - x < 3 => Some(y - x),
            _ => None,
        })
        .collect::<Vec<_>>();

    let chains = diff_chain
        .iter()
        .enumerate()
        .fold(1, |count, (index, &curr)| {
            match (curr, diff_chain.get(index + 1), diff_chain.get(index + 2)) {
                (2, Some(2), _) => count,
                (2, Some(1), Some(2)) | (1, Some(2), Some(2)) => count * 2,
                (1, Some(1), Some(2)) | (1, Some(2), Some(1)) => count * 3,
                x => panic!("{:?} not a valid pattern", x),
            }
        });

    chains
}

pub fn part_2() {
    let raw = fs::read_to_string("./src/day10.txt").unwrap();
    let parsed = parse_str(&raw);

    let result = count_possible_chains(parsed);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const RAW_DATA: &str = indoc!(
        "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4"
    );

    const LARGER_EXAMPLE: &str = indoc!(
        "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3"
    );

    #[test]
    fn first_not_sum_of_preable() {
        let parsed = super::parse_str(RAW_DATA);
        let result = super::one_and_three_jolt_differences(parsed);

        assert_eq!(result, (7, 5))
    }

    #[test]
    fn count_possible_chains() {
        let parsed = super::parse_str(RAW_DATA);
        let result = super::count_possible_chains(parsed);

        assert_eq!(result, 8);
    }

    #[test]
    fn count_possible_chains_bigger() {
        let parsed = super::parse_str(LARGER_EXAMPLE);
        let result = super::count_possible_chains(parsed);
        assert_eq!(result, 19208)
    }
}
