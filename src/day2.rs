use counter::Counter;
use regex::Regex;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
pub const DAY: u16 = 2;

#[derive(Debug, PartialEq)]
struct Policy {
    lower: usize,
    upper: usize,
    character: char,
}

#[derive(Debug, PartialEq)]
struct PolicyAndPass {
    policy: Policy,
    password: String,
}

impl PolicyAndPass {
    fn is_password_valid(&self) -> bool {
        let char_counts = self.password.chars().collect::<Counter<_>>();

        let &count = char_counts.get(&self.policy.character).unwrap_or(&0);
        !(count >= self.policy.lower && self.policy.upper >= count)
    }

    fn is_password_valid_ii(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        println!("{:?}", chars);
        let policy_char = Some(&self.policy.character);
        match (
            chars.get(self.policy.lower - 1) == policy_char,
            (chars.get(self.policy.upper - 1)) == policy_char,
        ) {
            (true, false) => true,
            (false, true) => true,
            _ => false,
        }
    }
}

impl FromStr for PolicyAndPass {
    type Err = ParseIntError;
    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
            (?P<lower>\d+) # lower bound of repetitions
            -
            (?P<upper>\d+) # upper bound of repetitions
            \s
            (?P<letters>\w) # letter(s?)
            :\s
            (?P<password>\w+)$
            ",
            )
            .unwrap();
        }

        let cap = RE.captures(line).unwrap();
        let lower = &cap[1];
        let lower = lower.parse::<usize>()?;
        let upper = &cap[2];
        let upper = upper.parse::<usize>()?;
        let raw_letters = &cap[3];
        let password = cap[4].to_string();

        Ok(PolicyAndPass {
            policy: Policy {
                lower,
                upper,
                character: raw_letters.chars().next().unwrap(),
            },
            password,
        })
    }
}

fn parse_str(s: &str) -> Vec<PolicyAndPass> {
    let text = s.lines();

    text.into_iter()
        .map(|l| l.parse::<PolicyAndPass>().unwrap())
        .collect()
}

fn parse() -> Vec<PolicyAndPass> {
    let raw = fs::read_to_string("./src/day2.txt").unwrap();
    parse_str(&raw)
}

pub fn part_1() {
    let data = parse();
    let valid = data
        .into_iter()
        .filter(|pp| pp.is_password_valid())
        .collect::<Vec<_>>();
    println!("Valid count: {}", valid.len())
}

pub fn part_2() {
    let data = parse();
    let valid = data
        .into_iter()
        .filter(|pp| pp.is_password_valid_ii())
        .collect::<Vec<_>>();
    println!("Valid count: {}", valid.len())
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse() {
        let line = "5-10 v: vvbvsvtmtvvvvv";
        let res = line.parse::<super::PolicyAndPass>();

        assert_eq!(
            super::PolicyAndPass {
                policy: super::Policy {
                    lower: 5,
                    upper: 10,
                    character: 'v'
                },
                password: "vvbvsvtmtvvvvv".to_string()
            },
            res.unwrap()
        );
    }

    #[test]
    fn is_valid() {
        let line = "5-10 v: vvbvsvtmtvvvvv";
        let res = line.parse::<super::PolicyAndPass>().unwrap();
        assert_eq!(res.is_password_valid(), true)
    }

    #[test]
    fn is_invalid() {
        let line = "1-3 b: cdefg";
        let res = line.parse::<super::PolicyAndPass>().unwrap();
        assert_eq!(res.is_password_valid(), false)
    }

    #[test]
    fn is_valid_ii() {
        let line = "1-3 a: abcde";
        let res = line.parse::<super::PolicyAndPass>().unwrap();
        assert_eq!(res.is_password_valid_ii(), true)
    }

    #[test]
    fn is_invalid_ii() {
        let line = "1-3 b: cdefg";
        let res = line.parse::<super::PolicyAndPass>().unwrap();
        assert_eq!(res.is_password_valid_ii(), false)
    }
}
