use aoc_2024::read_input_v1;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coord = (isize, isize);

#[derive(Debug)]
struct Input {
    ncol: usize,
    nrow: usize,
    antennas: HashMap<char, HashSet<Coord>>,
}

impl Input {
    fn get_antinodes(&self, p1: Coord, p2: Coord) -> Vec<Coord> {
        let mut res = vec![];
        let d = (p2.0 - p1.0, p2.1 - p1.1);
        let n1 = (p2.0 + d.0, p2.1 + d.1);
        let n2 = (p1.0 - d.0, p1.1 - d.1);

        if n1.0 >= 0 && n1.1 >= 0 && n1.0 < self.nrow as isize && n1.1 < self.ncol as isize {
            res.push(n1);
        }

        if n2.0 >= 0 && n2.1 >= 0 && n2.0 < self.nrow as isize && n2.1 < self.ncol as isize {
            res.push(n2);
        }

        res
    }

    fn get_antinodes_with_resonance(&self, p1: Coord, p2: Coord) -> Vec<Coord> {
        let mut res = vec![];
        let d = (p2.0 - p1.0, p2.1 - p1.1);
        let mut n1 = p1;
        let mut n2 = p2;

        while n1.0 >= 0 && n1.1 >= 0 && n1.0 < self.nrow as isize && n1.1 < self.ncol as isize {
            res.push(n1);
            n1 = (n1.0 + d.0, n1.1 + d.1);
        }

        while n2.0 >= 0 && n2.1 >= 0 && n2.0 < self.nrow as isize && n2.1 < self.ncol as isize {
            res.push(n2);
            n2 = (n2.0 - d.0, n2.1 - d.1);
        }

        res
    }

    fn parse(input: &str) -> Self {
        let mut ncol = 0;
        let mut antennas: HashMap<char, HashSet<Coord>> = HashMap::new();
        let lines = input.lines().enumerate().map(|(r, l)| {
            ncol = l
                .trim()
                .chars()
                .enumerate()
                .map(|(c, s)| {
                    if s.is_alphanumeric() {
                        antennas
                            .entry(s)
                            .and_modify(|s| {
                                s.insert((r as isize, c as isize));
                            })
                            .or_insert_with(|| HashSet::from([(r as isize, c as isize)]));
                    }
                    s
                })
                .count();
        });

        let nrow = lines.count();

        Self {
            ncol,
            nrow,
            antennas,
        }
    }

    fn antinodes(&self) -> usize {
        let mut nodes = HashSet::new();

        for (c, poses) in self.antennas.iter() {
            for pv in poses.iter().combinations(2) {
                for node in self.get_antinodes_with_resonance(*pv[0], *pv[1]) {
                    nodes.insert(node);
                }
            }

            println!("{c}: {:?}", poses.iter().combinations(2).count());
        }

        // for (_, poses) in self.antennas.iter() {
        //     for pv in poses.iter() {
        //         nodes.remove(pv);
        //     }
        // }

        for r in 0..self.nrow {
            for c in 0..self.ncol {
                if nodes.contains(&(r as isize, c as isize)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }

        nodes.len()
    }
}

fn main() {
    let input = &read_input_v1(8);
    // let input = "............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............";

    // ##....#....#
    // .#.#....0...
    // ..#.#0....#.
    // ..##...0....
    // ....0....#..
    // .#...#A....#
    // ...#..#.....
    // #....#.#....
    // ..#.....A...
    // ....#....A..
    // .#........#.
    // ...#......##

    let p = Input::parse(input);
    println!("{}", p.antinodes());
}
