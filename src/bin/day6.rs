use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
};

use aoc_2024::read_input_v1;

#[derive(Default, Debug)]
struct State {
    ncol: usize,
    nrow: usize,
    guard_pos: (usize, usize),
    guard_dir: (isize, isize),
    walls: HashSet<(usize, usize)>,
}

impl State {
    fn from_input(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|s| s.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let nrow = rows.len();
        let ncol = rows[0].len();
        let mut guard_pos = (0, 0);
        let mut guard_dir = (0, 0);
        let mut walls = HashSet::new();

        for (r, row) in rows.iter().enumerate() {
            for (c, &char) in row.iter().enumerate() {
                match char {
                    '#' => {
                        walls.insert((r, c));
                    }
                    '.' => {}
                    char => {
                        guard_pos = (r, c);
                        match char {
                            '^' => guard_dir = (-1, 0),
                            '<' => guard_dir = (0, -1),
                            'v' => guard_dir = (1, 0),
                            '>' => guard_dir = (0, 1),
                            c => unimplemented!("pls handle {c} ?"),
                        }
                    }
                }
            }
        }

        Self {
            ncol,
            nrow,
            guard_dir,
            guard_pos,
            walls,
        }
    }

    fn walk(
        &self,
        wall: Option<(usize, usize)>,
    ) -> (HashMap<(usize, usize), HashSet<(isize, isize)>>, bool) {
        let mut walking_set = HashMap::new();
        let mut was_loop = false;
        let mut pos = self.guard_pos;
        let mut dir = self.guard_dir;

        loop {
            let (gr, gc) = pos;
            let (dr, dc) = dir;
            if gr >= self.nrow || gc >= self.ncol {
                break;
            }

            walking_set
                .entry(pos)
                .or_insert_with(HashSet::new)
                .insert(dir);

            let (nr, or) = gr.overflowing_add_signed(dr);
            let (nc, oc) = gc.overflowing_add_signed(dc);

            if or || oc {
                break;
            }

            if self.walls.contains(&(nr, nc)) || wall.is_some() && wall.unwrap() == (nr, nc) {
                dir = (dc, -dr);
                continue;
            }

            pos = (nr, nc);

            if let Some(dir_set) = walking_set.get(&pos) {
                if dir_set.contains(&dir) {
                    was_loop = true;
                    break;
                }
            }
        }

        // if was_loop {
        //     for r in 0..self.nrow {
        //         for c in 0..self.ncol {
        //             if self.walls.contains(&(r, c)) {
        //                 print!("#");
        //             } else if walking_set.contains_key(&(r, c)) {
        //                 print!("X");
        //             } else if (r, c) == pos {
        //                 print!("G");
        //             } else {
        //                 print!(".")
        //             }
        //             print!(" ")
        //         }
        //         println!();
        //     }
        // }

        (walking_set, was_loop)
    }
}

fn solve1(input: &str) -> usize {
    let state = State::from_input(input);
    let (result, _) = state.walk(None);
    result.len()
}

fn solve2(input: &str) -> usize {
    let state = Arc::new(State::from_input(input));
    let count = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];
    let progress = Arc::new(Mutex::new(0));

    let (all, _) = state.walk(None);

    let total = all.len();
    for &(r, c) in all.keys() {
        let count = Arc::clone(&count);
        let state = Arc::clone(&state);
        let progress = Arc::clone(&progress);

        handle_vec.push(thread::spawn(move || {
            if (r, c) == state.guard_pos {
                return;
            }
            let (_, was_loop) = state.walk(Some((r, c)));
            if was_loop {
                *count.lock().unwrap() += 1;
            }
            *progress.lock().unwrap() += 1;
            println!("{} / {}", progress.lock().unwrap(), total);
        }));
    }

    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    {
        let result = *count.lock().unwrap();
        result
    }
}

fn main() {
    let input = &read_input_v1(6);
    // let input = "....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...";
    // println!("{}", solve1(input));
    println!("{}", solve2(input));
}
