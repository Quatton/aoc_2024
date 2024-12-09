use std::{
    sync::{Arc, Mutex},
    thread,
};

use aoc_2024::read_input_v1;

#[derive(Clone)]
struct Equation {
    target: usize,
    elements: Vec<usize>,
}

impl Equation {
    fn check(&self) -> Option<String> {
        let mut stack = (0..self.elements.len())
            .map(|idx| {
                let mut elements = self.elements.clone();
                elements.reverse();
                let first = elements[idx];
                elements.remove(idx);
                (first, elements, format!("{first}"))
            })
            .collect::<Vec<_>>();
        while let Some((curr, mut rest, str)) = stack.pop() {
            match rest.pop() {
                None => {
                    if rest.is_empty() {
                        if curr == self.target {
                            return Some(str);
                        }
                        continue;
                    }

                    if curr >= self.target {
                        continue;
                    }
                }
                Some(next) => {
                    let sum = curr + next;
                    let prod = curr * next;
                    let concat = format!("{curr}{next}").parse::<usize>().unwrap();

                    if sum <= self.target {
                        stack.push((sum, rest.clone(), format!("{str} + {next}")));
                    }

                    if prod <= self.target {
                        stack.push((prod, rest.clone(), format!("{str} * {next}")));
                    }

                    if concat <= self.target {
                        stack.push((concat, rest, format!("{str} || {next}")));
                    }
                }
            }
        }

        None
    }
}

struct Input {
    eqs: Vec<Equation>,
}

impl Input {
    fn parse(input: &str) -> Self {
        Self {
            eqs: input
                .lines()
                .map(|l| {
                    let (target, elements) = l.split_once(":").unwrap();
                    let target = target.trim().parse::<usize>().unwrap();
                    let elements = elements
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

                    Equation { target, elements }
                })
                .collect::<Vec<_>>(),
        }
    }
}

static NTHREAD: usize = 10;

fn main() {
    let input = &read_input_v1(7);
    //     let input = "190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20";

    let parsed = Input::parse(input);
    let mut threads = Vec::new();

    let sum = Arc::new(Mutex::new(0));

    for eqs_chunk in parsed.eqs.chunks(parsed.eqs.len() / NTHREAD) {
        let eqs = eqs_chunk.to_vec();
        let sum = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            for eq in eqs {
                if eq.check().is_some() {
                    *sum.lock().unwrap() += eq.target;
                }
            }
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    let sum = sum.lock().unwrap();
    println!("{sum}");
}
