use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::Infallible;
use std::fs;
use std::str::FromStr;

pub const DAY: u16 = 4;

#[derive(Clone, Debug, PartialEq)]
enum Identity {
    Passport(BTreeMap<String, String>),
    Invalid(BTreeMap<String, String>),
}

type ValidatorFn = Box<fn(String) -> Result<(), String>>;

struct Validator {
    key: String,
    f: ValidatorFn,
}

impl Identity {
    fn is_valid(&self) -> bool {
        let validators: Vec<Validator> = vec![
            Validator {
                key: "byr".to_string(),
                // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                f: Box::new(|s| match s.parse::<u32>() {
                    Ok(num) => match num {
                        1920..=2002 => Ok(()),
                        _ => Err("Outside range".to_string()),
                    },
                    Err(e) => Err(e.to_string()),
                }),
            },
            Validator {
                key: "iyr".to_string(),

                // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                f: Box::new(|s| match s.parse::<u32>() {
                    Ok(num) => match num {
                        2010..=2020 => Ok(()),
                        _ => Err("Outside range".to_string()),
                    },
                    Err(e) => Err(e.to_string()),
                }),
            },
            Validator {
                key: "eyr".to_string(),
                // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                f: Box::new(|s| match s.parse::<u32>() {
                    Ok(num) => match num {
                        2020..=2030 => Ok(()),
                        _ => Err("Outside range".to_string()),
                    },
                    Err(e) => Err(e.to_string()),
                }),
            },
            Validator {
                key: "hgt".to_string(),
                // hgt (Height) - a number followed by either cm or in:
                // If cm, the number must be at least 150 and at most 193.
                // If in, the number must be at least 59 and at most 76.
                f: Box::new(|s| {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$",).unwrap();
                    }

                    let captures = RE.captures(&s);

                    if captures.is_none() {
                        return Err(format!("{} doesn't match regex", s));
                    }

                    let matches = captures.unwrap();

                    match matches[1].parse::<u32>() {
                        Ok(num) => match (num, &matches[2]) {
                            (150..=193, "cm") => Ok(()),
                            (59..=76, "in") => Ok(()),
                            _ => Err("Outside range".to_string()),
                        },
                        Err(e) => Err(e.to_string()),
                    }
                }),
            },
            Validator {
                key: "hcl".to_string(),
                // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                f: Box::new(|s| {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
                    }

                    if RE.is_match(&s) {
                        return Ok(());
                    }

                    Err(format!("{} doesn't match regex", s))
                }),
            },
            Validator {
                key: "ecl".to_string(),
                // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                f: Box::new(|s| match s.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
                    _ => Err(s),
                }),
            },
            // TODO regex
            Validator {
                key: "pid".to_string(),
                // pid (Passport ID) - a nine-digit number, including leading zeroes.
                f: Box::new(|s| {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^(\d{9})$").unwrap();
                    }

                    if RE.is_match(&s) {
                        return Ok(());
                    }

                    Err(format!("{} doesn't match regex", s))
                }),
            },
        ];
        if let Identity::Passport(m) = &self {
            return validators.into_iter().all(|Validator { key, f }| {
                let raw = &m[&key];

                let result = f(raw.to_string());
                if result.is_err() {
                    return false;
                }

                true
            });
        }

        false
    }
}

impl FromStr for Identity {
    type Err = Infallible;
    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        let tokens = lines.split_ascii_whitespace();

        let required_fields = btreeset! {
            "byr",
            "ecl",
            "eyr",
            "hcl",
            "hgt",
            "iyr",
            "pid",
        };

        let mut kv = btreemap! {};

        for (key, val) in tokens.filter_map(|t| {
            let split = t.split(':').collect::<Vec<_>>();
            match split[..] {
                [key, val] => Some((key, val)),
                _ => None,
            }
        }) {
            kv.insert(key, val);
        }

        let current_keys = kv.keys().copied().collect::<BTreeSet<_>>();
        let result = kv
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<BTreeMap<_, _>>();

        if required_fields.is_subset(&current_keys) {
            return Ok(Identity::Passport(result));
        }

        Ok(Identity::Invalid(result))
    }
}

fn parse_str(s: &str) -> Vec<Identity> {
    // Segment
    let text = s.split("\n\n");

    text.into_iter()
        .map(|l| l.parse::<Identity>().unwrap())
        .collect()
}

fn parse() -> Vec<Identity> {
    let raw = fs::read_to_string("./src/day4.txt").unwrap();
    parse_str(&raw)
}

pub fn part_1() {
    let data = parse();

    let valid_passports = data
        .into_iter()
        .filter_map(|d| match d {
            Identity::Passport(_) => Some(()),
            Identity::Invalid(_) => None,
        })
        .count();

    println!("Valid passport count {}", valid_passports);
}

pub fn part_2() {
    let data = parse();

    let valid_passports = data
        .into_iter()
        .filter_map(|d| match d {
            Identity::Passport(_) if d.is_valid() => Some(()),
            _ => None,
        })
        .count();

    println!("Valid passport count {}", valid_passports);
}

#[cfg(test)]
mod tests {

    const RAW_DATA: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm

    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929

    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm

    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn parse() {
        let data = super::parse_str(RAW_DATA);

        assert!(match &data[0] {
            super::Identity::Passport(_) => true,
            x => panic!("Invalid variant {:?}", x),
        });

        assert!(match &data[1] {
            super::Identity::Invalid(_) => true,
            x => panic!("Invalid variant {:?}", x),
        });

        assert!(match &data[2] {
            super::Identity::Passport(_) => true,
            x => panic!("Invalid variant {:?}", x),
        });
    }
}
