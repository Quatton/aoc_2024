use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_2024::read_input_v1;

#[derive(Default)]
struct Grid {
    ncol: i32,
    nrow: i32,
    fb: HashSet<(i32, i32)>,
    fb_all: Vec<(i32, i32)>,
}

impl Grid {
    fn new(ncol: i32, nrow: i32) -> Self {
        Grid {
            ncol,
            nrow,
            ..Default::default()
        }
    }

    fn load(mut self, input: &str) -> Self {
        self.fb_all = input
            .lines()
            .map(|p| {
                let p = p
                    .trim()
                    .split(",")
                    .map(|p| p.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (p[0], p[1])
            })
            .collect();
        self
    }

    fn d(&self, p1: (i32, i32), p2: (i32, i32)) -> i32 {
        p1.0.abs_diff(p2.0) as i32 + p1.1.abs_diff(p2.1) as i32
    }

    fn h(&self, p: (i32, i32)) -> i32 {
        self.d(p, (self.nrow - 1, self.ncol - 1))
    }

    fn print_grid(&self) {
        for r in 0..self.nrow {
            for c in 0..self.ncol {
                if self.fb.contains(&(r, c)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn neighbours(&self, p: (i32, i32)) -> Vec<(i32, i32)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let x = p.0 as isize + dx;
                let y = p.1 as isize + dy;
                if x >= 0
                    && x < self.nrow as isize
                    && y >= 0
                    && y < self.ncol as isize
                    && !self.fb.contains(&(x as i32, y as i32))
                {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
            .collect()
    }

    fn fall(&mut self, num: usize) {
        self.fb = self.fb_all.iter().take(num).copied().collect();
    }

    fn solve(&self) -> Option<i32> {
        let mut open = BinaryHeap::from([Reverse((0, (0, 0)))]);
        let mut open_index = HashSet::from([(0, 0)]);

        let mut g_score = HashMap::from([((0, 0), 0)]);
        let mut f_score = HashMap::from([((0, 0), self.h((0, 0)))]);

        let end = (self.nrow - 1, self.ncol - 1);

        while let Some(Reverse((_, p))) = open.pop() {
            open_index.remove(&p);

            if p == end {
                break;
            }

            for n in self.neighbours(p) {
                let t_score = g_score[&p] + 1;

                if t_score < *g_score.get(&n).unwrap_or(&i32::MAX) {
                    g_score.insert(n, t_score);
                    f_score.insert(n, t_score + self.h(n));

                    if open_index.insert(n) {
                        open.push(Reverse((t_score, n)));
                    }
                }
            }
        }

        g_score.get(&end).copied()
    }
}

fn main() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    let input = &read_input_v1(18);

    let mut grid = Grid::new(71, 71).load(input);

    grid.fall(1024);
    println!("Part 1: {}", grid.solve().unwrap());

    for i in 1024.. {
        grid.fall(i);
        if grid.solve().is_none() {
            println!("Part 2: {}", i - 1);
            break;
        }
    }
}
