use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
    thread,
};

use aoc_2024::read_input_v1;
use itertools::Itertools;

fn gen(s: i64) -> i64 {
    let s = (s ^ s << 6) & 0xffffff;
    let s = (s ^ s >> 5) & 0xffffff;
    let s = (s ^ s << 11) & 0xffffff;
    s
}

fn gen_times(s: i64, c: usize, cache: &mut HashMap<(i64, usize), i64>) -> i64 {
    if c == 0 {
        return s;
    }

    match cache.get(&(s, c)) {
        Some(&u) => u,
        None => {
            let u = gen(gen_times(s, c - 1, cache));
            cache.insert((s, c - 1), u);
            u
        }
    }
}

fn main() {
    let input = "1
2
3
2024";
    let input = read_input_v1(22);

    let is: Vec<i64> = input
        .trim()
        .lines()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let subsq_count = Arc::new(Mutex::new(HashMap::<(i64, i64, i64, i64), _>::new()));

    let mut hds = vec![];

    for chunk in is.chunks(if is.len() / 10 < 1 { 1 } else { is.len() / 10 }) {
        let chunk = chunk.to_vec();
        let subsq_count = Arc::clone(&subsq_count);
        let handle = thread::spawn(move || {
            for l in chunk {
                let mut u = gen(l);
                let mut sq = VecDeque::new();
                let mut subsq = HashSet::<(i64, i64, i64, i64)>::new();

                for _ in 1..2000 {
                    let u1 = gen(u);
                    sq.push_back(u1 % 10 - u % 10);
                    u = u1;
                    if sq.len() >= 4 {
                        if subsq.insert((sq[0], sq[1], sq[2], sq[3])) {
                            *subsq_count
                                .lock()
                                .unwrap()
                                .entry((sq[0], sq[1], sq[2], sq[3]))
                                .or_insert(0) += u1 % 10;
                        }
                        sq.pop_front();
                    }
                }
            }
        });

        hds.push(handle);
    }

    for h in hds {
        h.join().unwrap();
    }

    let subsq_count = subsq_count.lock().unwrap();

    let c = subsq_count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

    println!("{:?}", c);
}
