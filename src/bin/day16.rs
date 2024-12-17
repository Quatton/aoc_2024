use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    i32,
};

use aoc_2024::read_input_v1;

static WALL: char = '#';

#[derive(Debug, Default)]
struct Maze {
    maze: Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
    face: (i32, i32),
}

type Camefrom = HashMap<((i32, i32), (i32, i32)), (i32, HashSet<((i32, i32), (i32, i32))>)>;

impl Maze {
    fn from_str(input: &str) -> Self {
        let mut pos = None;
        let mut end = None;
        let face = (0, 1);
        let maze = input
            .lines()
            .enumerate()
            .map(|(r, l)| {
                let p = l.replace(" ", "");

                for (c, col) in p.chars().enumerate() {
                    match col {
                        'S' => {
                            pos = Some((r as i32, c as i32));
                        }
                        'E' => {
                            end = Some((r as i32, c as i32));
                        }
                        _ => {}
                    }
                }

                p.chars().collect()
            })
            .collect();

        let pos = pos.unwrap();
        let end = end.unwrap();

        Self {
            maze,
            start: pos,
            end,
            face,
            ..Default::default()
        }
    }

    fn d(&self, pf: &(i32, i32), prev: &(i32, i32), nf: &(i32, i32), next: &(i32, i32)) -> i32 {
        if pf != nf && prev != next {
            panic!("not allowing turn and step");
        }

        if pf == nf {
            return 1;
        }

        // (1, 0) -> (0, 1)
        // (1, 0) -> (0, -1)

        // 90 -> 0
        //  0 > 0
        // 180 < 0

        let dot = pf.0 * nf.0 + pf.1 * nf.1;

        1000 * (match dot.cmp(&0) {
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Greater => 0,
        })
    }

    fn h(&self, next: &(i32, i32)) -> i32 {
        (next.0 - self.end.0).abs() + (next.1 - self.end.1).abs()
    }

    fn neightbour(&self, face: &(i32, i32), pos: &(i32, i32)) -> Vec<((i32, i32), (i32, i32))> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(dr, dc)| {
                if *face == (-dr, -dc) {
                    return None;
                }

                if (dr, dc) == *face {
                    let next = (pos.0 + face.0, pos.1 + face.1);
                    if self.maze[next.0 as usize][next.1 as usize] == WALL {
                        return None;
                    }
                    Some(((dr, dc), next))
                } else {
                    Some(((dr, dc), *pos))
                }
            })
            .collect()
    }

    fn collect_paths(
        &self,
        came_from: &Camefrom,
        face: (i32, i32),
        end: (i32, i32),
    ) -> HashSet<(i32, i32)> {
        let mut paths = HashSet::from([(face, end)]);

        let mut stack = vec![(face, end)];

        while let Some(p) = stack.pop() {
            if let Some((_, set)) = came_from.get(&p) {
                for p in set {
                    paths.insert(*p);
                    stack.push(*p);
                }
            }
        }

        paths.iter().map(|(_, p)| *p).collect()
    }

    fn solve(
        &self,
        face: (i32, i32),
        start: (i32, i32),
        end: (i32, i32),
    ) -> (Camefrom, Option<(((i32, i32), (i32, i32)), i32)>) {
        let f_start = self.h(&start);
        let mut open = BinaryHeap::from([Reverse((f_start, face, start))]);
        let mut open_index = HashSet::from([(face, start)]);

        let mut g_score = HashMap::from([((face, start), 0)]);
        let mut f_score = HashMap::from([((face, start), f_start)]);

        let mut came_from: Camefrom = HashMap::new();

        while let Some(Reverse((_, face, pos))) = open.pop() {
            open_index.remove(&(face, pos));

            for (n_face, next) in self.neightbour(&face, &pos) {
                let d = self.d(&face, &pos, &n_face, &next);
                let t_score = g_score.get(&(face, pos)).unwrap() + d;
                let r_score = *g_score.get(&(n_face, next)).unwrap_or(&i32::MAX);

                if t_score < r_score {
                    g_score.insert((n_face, next), t_score);
                    f_score.insert((n_face, next), t_score + self.h(&next));

                    let (p_score, set) = came_from
                        .entry((n_face, next))
                        .or_insert_with(|| (i32::MAX, HashSet::new()));

                    if t_score < *p_score {
                        set.clear();
                    }

                    set.insert((face, pos));

                    if open_index.insert((n_face, next)) {
                        open.push(Reverse((t_score, n_face, next)));
                    }
                }

                if t_score == r_score {
                    let (_, set) = came_from
                        .entry((n_face, next))
                        .or_insert_with(|| (i32::MAX, HashSet::new()));

                    set.insert((face, pos));
                }
            }
        }

        (
            came_from,
            g_score
                .iter()
                .filter(|((_, pos), _)| *pos == end)
                .min_by(|a, b| a.1.cmp(b.1))
                .map(|(p, c)| (*p, *c)),
        )
    }

    // fn through(
    //   &self,
    //   face: (i32, i32),
    //   start: (i32, i32),
    //   end: (i32, i32),
    //   through: (i32, i32),
    // ) {
    //   let (ca1, co1) = self.solve(face, start, end)
    // }
}

fn main() {
    let input = "###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############";
    let input = &read_input_v1(16);

    let mut m = Maze::from_str(input);

    let (came_from, cost) = m.solve(m.face, m.start, m.end);

    if cost.is_none() {
        println!("No path found");
        return;
    }

    let ((face, pos), cost) = cost.unwrap();
    println!("{:?} {:?}", face, pos);
    println!("{:?}", cost);

    // for (p, set) in came_from.iter() {
    //     if set.len() > 1 {
    //         println!("{:?} {:?}", p, set);
    //     }
    // }

    let tiles = m.collect_paths(&came_from, face, m.end);

    for (r, row) in m.maze.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == WALL {
                print!("{}", WALL);
                continue;
            }

            if tiles.contains(&(r as i32, c as i32)) {
                print!("O");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!("{:?}", tiles.len());
}
