use std::collections::{HashMap, HashSet};

use aoc_2024::read_input_v1;

#[derive(Debug)]
struct Input {
    rules: HashSet<String>,
    checklist: Vec<String>,
    cache: HashMap<String, (usize, usize)>,
}

impl Input {
    fn from_str(input: &str) -> Self {
        let (towels, checklist) = input.split_once("\n\n").unwrap();
        let rules = towels
            .trim()
            .split(',')
            .map(|l| l.trim().to_string())
            .collect();
        let checklist = checklist
            .trim()
            .lines()
            .map(|l| l.trim().to_string())
            .collect();

        Self {
            rules,
            checklist,
            cache: HashMap::new(),
        }
    }

    fn check(&mut self, input: &str) -> usize {
        let mut chart = vec![vec![(0_usize, 0_usize); input.len()]; input.len()];

        for r in (0..input.len()).rev() {
            for c in 0..=r {
                let substr = &input[c..c + input.len() - r];

                if let Some(cached) = self.cache.get(substr) {
                    chart[r][c] = *cached;
                    continue;
                }

                if self.rules.contains(substr) {
                    chart[r][c] = (1, 1);
                }

                for k in (r + 1..input.len()).rev() {
                    let p1 = chart[k][c].0;
                    let p2 = chart[input.len() + r - k][c + input.len() - k].1;

                    if p1 == 0 || p2 == 0 {
                        continue;
                    }

                    chart[r][c].1 += p1 * p2;
                }

                self.cache.insert(substr.to_string(), chart[r][c]);
            }
        }

        chart[0][0].1
    }

    fn check_all(&mut self) -> usize {
        let list = self.checklist.clone();
        list.iter().filter(|c| self.check(c) > 0).count()
    }

    fn count_all(&mut self) -> usize {
        let list = self.checklist.clone();
        list.iter().map(|c| self.check(c)).sum()
    }
}

fn main() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    let input = &read_input_v1(19);
    let mut p = Input::from_str(input);

    println!("Part 1: {}", p.check_all());
    // println!("brwrr: {}", p.check("brwrr"));
    // println!("bggr: {}", p.check("bggr"));
    // println!("gbbr: {}", p.check("gbbr"));
    // println!("rrbgbr: {}", p.check("rrbgbr"));

    println!("Part 2: {}", p.count_all());
}
