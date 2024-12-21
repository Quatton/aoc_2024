use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_2024::read_input_v1;
use itertools::Itertools;

#[derive(Debug, Default)]
struct Maze {
    nrow: i32,
    ncol: i32,
    map: Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
}

static WALL: char = '#';

impl Maze {
    fn from_str(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map: Vec<_> = input
            .lines()
            .enumerate()
            .map(|(r, l)| {
                let l = l.trim().chars().collect::<Vec<_>>();

                for (c, ch) in l.iter().enumerate() {
                    let pos = (r as i32, c as i32);
                    match ch {
                        'S' => start = pos,
                        'E' => end = pos,
                        _ => {}
                    }
                }

                l
            })
            .collect();

        Self {
            start,
            end,
            nrow: map.len() as i32,
            ncol: map[0].len() as i32,
            map,
            ..Default::default()
        }
    }

    fn neighbours(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbours = vec![];

        for (r, c) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + r, pos.1 + c);

            if new_pos == self.end {
                neighbours.push(new_pos);
                break;
            }

            if self.map[new_pos.0 as usize][new_pos.1 as usize] == WALL {
                continue;
            }

            if new_pos.0 >= 0 && new_pos.0 < self.nrow && new_pos.1 >= 0 && new_pos.1 < self.ncol {
                neighbours.push(new_pos);
            }
        }

        neighbours
    }

    fn solve(&self, target: i32, seconds: i32) -> i32 {
        let mut g_score = HashMap::from([(self.start, 0)]);
        let mut open = BinaryHeap::new();

        open.push(Reverse((0, self.start)));

        while let Some(Reverse((_, pos))) = open.pop() {
            if pos == self.end {
                break;
            }
            for n in self.neighbours(pos) {
                let t_score = g_score.get(&pos).unwrap_or(&0) + 1;

                if t_score < *g_score.get(&n).unwrap_or(&i32::MAX) {
                    g_score.insert(n, t_score);
                    open.push(Reverse((t_score, n)));
                }
            }
        }

        g_score
            .keys()
            .sorted()
            .combinations(2)
            .filter(|a| {
                let skipped = (a[0].0.abs_diff(a[1].0) + a[0].1.abs_diff(a[1].1)) as i32;
                skipped <= seconds
                    && g_score[a[0]].abs_diff(g_score[a[1]]) as i32 - skipped >= target
            })
            .count() as i32
    }
}

fn main() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    let input = &read_input_v1(20);

    let maze = Maze::from_str(input);

    // for i in 50..77 {
    println!("Cheats: {}", maze.solve(100, 20));
    // }
}
