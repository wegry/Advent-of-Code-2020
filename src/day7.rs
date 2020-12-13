use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fs;
use std::str::FromStr;

use regex::Regex;

pub const DAY: u16 = 7;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    name: String,
    count_by_child: BTreeMap<String, usize>,
}

fn normalize_bag_name(s: &str) -> String {
    s.trim_end_matches('s').trim_end_matches(" bag").to_string()
}

impl FromStr for Rule {
    type Err = Infallible;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+) (.*)$",).unwrap();
        }
        let split = raw.split(" contain ").collect::<Vec<_>>();
        let name = normalize_bag_name(split[0]);
        let mut count_by_child = btreemap! {};
        for s in split[1].trim_end_matches('.').split(", ") {
            if s == "no other bags" {
                return Ok(Rule {
                    name,
                    count_by_child: btreemap! {},
                });
            }
            let captures = RE.captures(s).unwrap();
            let count = captures[1].parse::<usize>().unwrap();
            let child_name = normalize_bag_name(&captures[2]);

            count_by_child.insert(child_name.to_string(), count);
        }

        Ok(Rule {
            name,
            count_by_child,
        })
    }
}

type FlattenedGraph = BTreeMap<String, BTreeMap<String, usize>>;

fn parse_str(raw: &str) -> FlattenedGraph {
    raw.lines()
        .map(|l| {
            let Rule {
                name,
                count_by_child,
            } = l.parse::<Rule>().unwrap();
            (name, count_by_child)
        })
        .collect()
}

pub fn part_1() {
    let raw = fs::read_to_string("./src/day7.txt").unwrap();
    let rule_map = parse_str(&raw);

    let mut new_additions = btreeset! {"shiny gold".to_string()};
    let mut result = btreeset! {};

    loop {
        let mut newer_additions = btreeset! {};
        for item in new_additions.clone() {
            let matches = rule_map
                .iter()
                .filter_map(|(name, children)| {
                    if children.contains_key(&item) {
                        Some(name.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            newer_additions = newer_additions
                .union(&matches)
                .map(|s| s.to_string())
                .collect();
        }

        result = result
            .union(&newer_additions)
            .map(|s| s.to_string())
            .collect();

        if new_additions == newer_additions {
            break;
        }

        new_additions = newer_additions.clone();
    }

    println!(
        "There are {} potentially encompassing bag colors",
        result.len(),
    );
}

fn count_contained_helper(g: &FlattenedGraph, counting: String) -> usize {
    let current = &g[&counting];

    current
        .iter()
        .map(|(key, value)| value * (count_contained_helper(g, key.clone()) + 1))
        .sum::<usize>()
}

fn count_contained(raw: &str) -> usize {
    let rule_map = parse_str(&raw);

    count_contained_helper(&rule_map, "shiny gold".to_string())
}

pub fn part_2() {
    let raw = fs::read_to_string("./src/day7.txt").unwrap();
    let result = count_contained(&raw);

    println!("There are {} potentially encompassed bags", result,);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const RAW_DATA: &str = indoc!(
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
    );
    #[test]
    fn parse() {
        let parsed = RAW_DATA
            .lines()
            .map(|l| l.parse::<super::Rule>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(parsed[0].name, "light red");
        assert_eq!(parsed[0].count_by_child["bright white"], 1);
    }

    #[test]
    fn count_contained() {
        let counted = super::count_contained(RAW_DATA);

        assert_eq!(counted, 32);

        let counted_2 = super::count_contained(indoc!(
            "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags."
        ));

        assert_eq!(counted_2, 126);
    }
}
