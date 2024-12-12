use std::{
    collections::HashSet,
    ops::{Add, Index},
};

use aoc_2024::read_input_v1;

struct Input {
    map: Vec<Vec<usize>>,
    nrow: usize,
    ncol: usize,
}

impl Index<Coord> for Input {
    type Output = usize;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.map[index.0 .0][index.0 .1]
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord((usize, usize));

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord((self.0 .0 + rhs.0 .0, self.0 .1 + rhs.0 .1))
    }
}

#[derive(Debug, Copy, Clone)]
struct SCoord((isize, isize));

impl Add for SCoord {
    type Output = SCoord;

    fn add(self, rhs: Self) -> Self::Output {
        SCoord((self.0 .0 + rhs.0 .0, self.0 .1 + rhs.0 .1))
    }
}

impl From<Coord> for SCoord {
    fn from(value: Coord) -> Self {
        SCoord((value.0 .0 as isize, value.0 .1 as isize))
    }
}

impl SCoord {
    fn to_coord(self) -> Result<Coord, ()> {
        if self.0 .0 < 0 || self.0 .1 < 0 {
            Err(())
        } else {
            Ok(Coord((self.0 .0 as usize, self.0 .1 as usize)))
        }
    }
}

impl Input {
    fn parse(input: &str) -> Self {
        let map = input
            .split_whitespace()
            .map(|line| {
                line.bytes()
                    .map(|b| (b - b'0') as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let nrow = map.len();
        let ncol = map[0].len();

        Self { map, nrow, ncol }
    }

    fn get_zeros(&self) -> Vec<Coord> {
        let mut coords = vec![];

        for r in 0..self.nrow {
            for c in 0..self.ncol {
                if self.map[r][c] == 0 {
                    coords.push(Coord((r, c)))
                }
            }
        }

        coords
    }

    fn get_neighbors(&self, pos: Coord) -> Vec<Coord> {
        let pos = SCoord::from(pos);
        (-1..=1).fold(vec![], |acc, dr| {
            (-1..=1).fold(acc, |mut acc, dc| {
                if (dr as i32).abs() + (dc as i32).abs() == 1 {
                    let new_pos: SCoord = pos + SCoord((dr, dc));
                    if let Ok(new_pos) = new_pos.to_coord() {
                        if new_pos.0 .0 < self.nrow && new_pos.0 .1 < self.ncol {
                            acc.push(new_pos);
                        }
                    }
                }

                acc
            })
        })
    }

    fn walk(&self, zero: Coord) -> HashSet<Coord> {
        let mut frontier = vec![zero];

        let mut nines = HashSet::new();

        while let Some(pos) = frontier.pop() {
            if self[pos] == 9 {
                nines.insert(pos);
                continue;
            }

            let neighbours = self.get_neighbors(pos);

            for n in neighbours {
                if self[n].saturating_sub(self[pos]) == 1 {
                    frontier.push(n)
                }
            }

            // if zero == Coord((5, 2)) {
            //     println!("{frontier:?}")
            // }
        }

        nines
    }

    fn walk_with_rating(&self, zero: Coord) -> (HashSet<Coord>, usize) {
        let mut frontier = vec![zero];

        let mut nines = HashSet::new();
        let mut nines_but_vec = vec![];

        while let Some(pos) = frontier.pop() {
            if self[pos] == 9 {
                nines.insert(pos);
                nines_but_vec.push(pos);
                continue;
            }

            let neighbours = self.get_neighbors(pos);

            for n in neighbours {
                if self[n].saturating_sub(self[pos]) == 1 {
                    frontier.push(n);
                }
            }
        }

        (nines, nines_but_vec.len())
    }

    fn total_trailheads(&self) -> usize {
        let zeros = self.get_zeros();

        zeros.iter().fold(0, |acc, cur| {
            let nines = self.walk(*cur);
            acc + nines.len()
        })
    }

    fn total_rating(&self) -> usize {
        let zeros = self.get_zeros();

        zeros.iter().fold(0, |acc, cur| {
            let nines = self.walk_with_rating(*cur);
            acc + nines.1
        })
    }
}

fn main() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let input = &read_input_v1(10);

    let p = Input::parse(input);
    // let nines = p.total_trailheads();
    let nines = p.total_rating();

    println!("{:?}", nines);
}
