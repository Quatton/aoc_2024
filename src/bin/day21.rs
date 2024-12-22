use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::repeat_n,
};

use itertools::Itertools;

static NUMPAD_V: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['X', '0', 'A'],
];

static DPAD_V: [[char; 3]; 2] = [['X', '^', 'A'], ['<', 'v', '>']];

fn simulate_dpad(input: &str, from: char) -> char {
    let mut pos = DPAD_V
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, &n)| {
                if n == from {
                    Some((r as i32, c as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    for ch in input.chars() {
        let (r, c) = pos;
        let (nr, nc) = match ch {
            '^' => (r - 1, c),
            'v' => (r + 1, c),
            '<' => (r, c - 1),
            '>' => (r, c + 1),
            'A' => (r, c),
            c => unreachable!("invalid char: {}", c),
        };

        if (0..2).contains(&nr) && (0..3).contains(&nc) && DPAD_V[nr as usize][nc as usize] != 'X' {
            pos = (nr, nc);
        }
    }

    DPAD_V[pos.0 as usize][pos.1 as usize]
}

fn build_dpad_map() -> HashMap<(char, char), HashSet<String>> {
    let mut res: HashMap<(char, char), HashSet<String>> = HashMap::new();

    let list = ['^', 'v', '<', '>', 'A'];
    for l in list.iter().copied().combinations_with_replacement(2) {
        let (from, to) = (l[0], l[1]);
        if from == to {
            res.insert((from, to), HashSet::from([String::new()]));
            continue;
        }

        let fromn = DPAD_V
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter().enumerate().find_map(|(c, &n)| {
                    if n == from {
                        Some((r as i32, c as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let ton = DPAD_V
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter().enumerate().find_map(|(c, &n)| {
                    if n == to {
                        Some((r as i32, c as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let diff_x = ton.1 - fromn.1;
        let diff_y = ton.0 - fromn.0;

        let h = repeat_n(
            if diff_x > 0 { '>' } else { '<' },
            diff_x.unsigned_abs() as usize,
        )
        .join("");

        let v = repeat_n(
            if diff_y > 0 { 'v' } else { '^' },
            diff_y.unsigned_abs() as usize,
        )
        .join("");

        // let moves = h
        //     .chain(v)
        //     .permutations(diff_x.unsigned_abs() as usize + diff_y.unsigned_abs() as usize)

        let moves = [h, v]
            .into_iter()
            .permutations(2)
            .filter_map(|m| {
                let m = m.into_iter().collect::<String>();
                if simulate_dpad(&m, from) == to {
                    Some(m)
                } else {
                    None
                }
            })
            .collect();

        res.insert((from, to), moves);

        res.insert(
            (to, from),
            res[&(from, to)].iter().map(|c| rev_path(c)).collect(),
        );
    }

    res
}

fn rev_path(path: &str) -> String {
    path.chars()
        .rev()
        .map(|c| match c {
            '>' => '<',
            '<' => '>',
            '^' => 'v',
            'v' => '^',
            'A' => 'A',
            _ => unreachable!(),
        })
        .collect::<String>()
}

fn build_numpad_map() -> HashMap<(char, char), HashSet<String>> {
    let mut res: HashMap<(char, char), HashSet<String>> = HashMap::new();

    for l in "0123456789A".chars().combinations_with_replacement(2) {
        let (fc, tc) = (l[0], l[1]);

        if fc == tc {
            res.insert((fc, tc), HashSet::from([String::new()]));
            continue;
        }

        let f = NUMPAD_V
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter().enumerate().find_map(|(c, &n)| {
                    if n == fc {
                        Some((r as i32, c as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let t = NUMPAD_V
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter().enumerate().find_map(|(c, &n)| {
                    if n == tc {
                        Some((r as i32, c as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let diff_x = t.1 - f.1;
        let diff_y = t.0 - f.0;

        let h = repeat_n(
            if diff_x > 0 { '>' } else { '<' },
            diff_x.unsigned_abs() as usize,
        )
        .join("");

        let v = repeat_n(
            if diff_y > 0 { 'v' } else { '^' },
            diff_y.unsigned_abs() as usize,
        )
        .join("");

        let moves = [h, v]
            .into_iter()
            .permutations(2)
            .filter_map(|m| {
                let m = m.into_iter().collect::<String>();
                if simulate_numpad(&m, fc) == tc {
                    Some(m)
                } else {
                    None
                }
            })
            .collect();

        res.insert((fc, tc), moves);

        res.insert(
            (tc, fc),
            res[&(fc, tc)].iter().map(|c| rev_path(c)).collect(),
        );
    }

    res
}

fn simulate_numpad(input: &str, from: char) -> char {
    let mut pos = NUMPAD_V
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, &n)| {
                if n == from {
                    Some((r as i32, c as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    for ch in input.chars() {
        let (r, c) = pos;
        let (nr, nc) = match ch {
            '^' => (r - 1, c),
            'v' => (r + 1, c),
            '<' => (r, c - 1),
            '>' => (r, c + 1),
            'A' => (r, c),
            _ => unreachable!(),
        };

        if (0..4).contains(&nr) && (0..3).contains(&nc) && NUMPAD_V[nr as usize][nc as usize] != 'X'
        {
            pos = (nr, nc);
        }
    }

    NUMPAD_V[pos.0 as usize][pos.1 as usize]
}

struct Map {
    numpad_map: HashMap<(char, char), HashSet<String>>,
    dpad_map: HashMap<(char, char), HashSet<String>>,
}

impl Map {
    fn new() -> Self {
        Self {
            numpad_map: build_numpad_map(),
            dpad_map: build_dpad_map(),
        }
    }

    fn join_numpad_moves(&self, input: &str) -> Vec<HashSet<String>> {
        let mut res = vec![];
        let mut last = 'A';

        for ch in input.chars() {
            let moves = self.numpad_map.get(&(last, ch)).unwrap();
            res.push(moves.iter().map(|m| format!("{}A", m)).collect());
            last = ch;
        }

        res
    }

    fn join_dpad_moves(&self, input: &str) -> Vec<HashSet<String>> {
        let mut res = vec![];
        let mut last = 'A';

        for ch in input.chars() {
            let moves = self
                .dpad_map
                .get(&(last, ch))
                .unwrap_or_else(|| panic!("{} -> {}", last, ch));
            res.push(moves.iter().map(|m| format!("{}A", m)).collect());
            last = ch;
        }

        res
    }

    fn all_numpad_moves(&self, input: &str) -> HashSet<String> {
        self.join_numpad_moves(input)
            .iter()
            .fold(HashSet::from(["".to_string()]), |acc, m| {
                let mut res = HashSet::new();
                for a in acc.iter() {
                    for b in m.iter() {
                        res.insert(format!("{}{}", a, b));
                    }
                }
                res
            })
    }

    fn all_dpad_moves(&self, input: &str) -> HashSet<String> {
        self.join_dpad_moves(input)
            .iter()
            .fold(HashSet::from(["".to_string()]), |acc, m| {
                let mut res = HashSet::new();
                for a in acc.iter() {
                    for b in m.iter() {
                        res.insert(format!("{}{}", a, b));
                    }
                }
                res
            })
    }

    fn solve(&self, input: &str) -> usize {
        let numpad_moves = self.all_numpad_moves(input);
        let dpad_moves = numpad_moves.iter().fold(HashSet::new(), |mut acc, m| {
            acc.extend(self.all_dpad_moves(m));
            acc
        });
        let dpad_dpad_moves = dpad_moves.iter().fold(HashSet::new(), |mut acc, m| {
            acc.extend(self.all_dpad_moves(m));
            acc
        });

        dpad_dpad_moves.into_iter().map(|m| m.len()).min().unwrap()
    }

    fn calculate_complexity(&self, input: &str, depth: usize) -> usize {
        let input_num = input.trim_end_matches('A').parse::<usize>().unwrap();
        let dpads = self.all_numpad_moves(input);
        let mut cache = HashMap::new();

        let complexity = dpads
            .iter()
            .map(|input| self.solve_depth(input.to_string(), &mut cache, depth))
            .min()
            .unwrap();

        complexity * input_num
    }

    fn solve_depth(
        &self,
        num: String,
        cache: &mut HashMap<(String, usize), usize>,
        count: usize,
    ) -> usize {
        if count == 0 {
            return num.len();
        }

        let len = match cache.get(&(num.clone(), count)) {
            Some(str) => *str,
            None => self
                .join_dpad_moves(&num)
                .iter()
                .map(|m| {
                    m.iter()
                        .map(|m| self.solve_depth(m.to_string(), cache, count - 1))
                        .min()
                        .unwrap_or(0)
                })
                .sum::<usize>(),
        };

        cache.insert((num, count), len);
        len
    }
}

fn main() {
    let input = "169A
279A
540A
869A
789A";

    let map = Map::new();

    let res = input
        .trim()
        .lines()
        .map(|l| map.calculate_complexity(l, 25))
        .sum::<usize>();

    println!("{}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn numpad_map_works() {
        let numpad_map = build_numpad_map();

        for from in 0..=10 {
            for to in 0..=10 {
                let from = if from == 10 {
                    'A'
                } else {
                    from.to_string().chars().next().unwrap()
                };

                let to = if to == 10 {
                    'A'
                } else {
                    to.to_string().chars().next().unwrap()
                };

                if from == to {
                    assert_eq!(simulate_numpad("", from), to, "no-op should work");
                }

                let moves = numpad_map
                    .get(&(from, to))
                    .unwrap_or_else(|| panic!("{} -> {}", from, to));

                for m in moves {
                    assert_eq!(
                        simulate_numpad(m, from),
                        to,
                        "from: {}, to: {}, via: {}",
                        from,
                        to,
                        m
                    );
                }
            }
        }
    }

    #[test]
    fn dpad_map_works() {
        let dpad_map = build_dpad_map();

        for from in ['^', 'v', '<', '>'] {
            for to in ['^', 'v', '<', '>'] {
                if from == to {
                    assert_eq!(simulate_dpad("", from), to, "no-op should work");
                }

                let moves = dpad_map
                    .get(&(from, to))
                    .unwrap_or_else(|| panic!("{} -> {}", from, to));

                for m in moves {
                    assert_eq!(
                        simulate_dpad(m, from),
                        to,
                        "from: {}, to: {}, via: {}",
                        from,
                        to,
                        m
                    );
                }
            }
        }
    }
}
