use std::{collections::HashSet, fmt::Debug};

use aoc_2024::read_input_v1;

struct Input {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Rule {
    after: HashSet<(usize, usize)>,
}

impl Rule {
    fn from_vec(vec: &[(usize, usize)]) -> Self {
        let after: HashSet<(usize, usize)> = HashSet::from_iter(vec.iter().copied());
        Self { after }
    }

    fn check_update(&self, vec: &[usize]) -> bool {
        for second in 0..vec.len() {
            for first in 0..second {
                let a = vec[second];
                let b = vec[first];
                if self.after.contains(&(a, b)) {
                    // println!("found {} after {} in {:?}", a, b, vec);
                    return false;
                }
            }
        }

        true
    }

    fn check_update_with_sort(&self, mut vec: Vec<usize>) -> Option<Vec<usize>> {
        if self.check_update(&vec) {
            return None;
        }

        let mut second = 1;
        while second < vec.len() {
            for first in 0..second {
                let a = vec[second];
                let b = vec[first];
                if self.after.contains(&(a, b)) {
                    vec.swap(first, second);
                    second = first;
                    break;
                }
            }
            second += 1;
        }

        if !self.check_update(&vec) {
            panic!("Failed to sort");
        }

        Some(vec)
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n1, n2) in self.rules.iter() {
            writeln!(f, "{n1}|{n2}")?;
        }

        writeln!(f)?;

        for u in self.updates.iter() {
            for v in u.iter() {
                write!(f, "{v}, ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
fn solve(input: &str) -> usize {
    let parsed = parse_input(input);

    let rules = Rule::from_vec(&parsed.rules);

    let passed = parsed
        .updates
        .iter()
        .filter_map(|u| {
            if rules.check_update(u) {
                Some(u.get(u.len() / 2))
            } else {
                None
            }
        })
        .map(|u| u.unwrap())
        .sum::<usize>();

    passed
}

fn solve2(input: &str) -> usize {
    let parsed = parse_input(input);

    let rules = Rule::from_vec(&parsed.rules);

    let passed = parsed
        .updates
        .iter()
        .filter_map(|u| rules.check_update_with_sort(u.to_vec()))
        .map(|u| {
            let mid = u.len() / 2;
            *u.get(mid).unwrap()
        })
        .sum::<usize>();

    passed
}
fn parse_input(input: &str) -> Input {
    let (r, u) = input.split_once("\n\n").unwrap();

    let rules = r
        .lines()
        .map(|p| {
            let (n1, n2) = p.split_once("|").unwrap();
            let n1 = n1.parse::<usize>().unwrap();
            let n2 = n2.parse::<usize>().unwrap();
            (n1, n2)
        })
        .collect::<Vec<_>>();

    let updates = u
        .lines()
        .map(|p| {
            p.split(",")
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Input { rules, updates }
}
fn main() {
    let input = &read_input_v1(5);
    println!("{}", solve2(input));
}

#[cfg(test)]
mod test {
    use crate::solve2;

    use super::*;

    #[test]
    fn test_parser() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let parsed = parse_input(input);

        let rules = Rule::from_vec(&parsed.rules);

        let passed = parsed
            .updates
            .iter()
            .filter_map(|u| {
                if rules.check_update(u) {
                    Some(u.get(u.len() / 2))
                } else {
                    None
                }
            })
            .map(|u| u.unwrap())
            .sum::<usize>();

        // println!("{:?}", parsed);
        // println!("{:?}", rules);
        println!("{passed:?}");
    }

    #[test]
    fn test_solve2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let parsed = solve2(input);
        println!("{parsed:?}");
    }
}
