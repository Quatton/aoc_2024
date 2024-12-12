use std::{collections::HashMap, thread};

use aoc_2024::read_input_v1;

#[derive(Debug, Default)]
struct Input {
    line: Vec<usize>,
}

impl Input {
    fn parse(input: &str) -> Self {
        Self {
            line: input
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn blink(&self, count: usize) -> usize {
        let mut blink_map = HashMap::new();

        self.line.iter().fold(0, |acc, &cur| {
            acc + blink_count_only(cur, count, &mut blink_map)
        })
    }
}

fn blink_count_only(
    num: usize,
    count: usize,
    blink_map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if count == 0 {
        return 1;
    }

    let len = match blink_map.get(&(num, count)) {
        Some(&len) => len,
        None => {
            let line = if num == 0 {
                vec![1]
            } else {
                let str = num.to_string();
                if str.len() % 2 == 0 {
                    let (left, right) = str.split_at(str.len() / 2);
                    vec![left.parse().unwrap(), right.parse().unwrap()]
                } else {
                    vec![num * 2024]
                }
            };

            line.iter().fold(0, |acc, &cur| {
                acc + blink_count_only(cur, count - 1, blink_map)
            })
        }
    };
    // println!("blink {num} for {count} times get {len}");
    blink_map.insert((num, count), len);
    len
}

fn main() {
    // let input = "125 17";
    let input = &read_input_v1(11);

    let p = Input::parse(input);
    let count = p.blink(75);
    println!("{count}");
}
