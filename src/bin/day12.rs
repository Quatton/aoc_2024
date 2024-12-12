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

use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Index, Range, RangeInclusive},
};

use aoc_2024::read_input_v1;

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

    fn get_regions(&self) -> Vec<Vec<Coord>> {
        let mut processed: Vec<Vec<bool>> = self
            .map
            .iter()
            .map(|p| p.iter().map(|_| false).collect())
            .collect();

        let mut regions = vec![];

        for r in 0..self.nrow {
            for c in 0..self.ncol {
                if processed[r][c] {
                    continue;
                }

                let mut region = vec![];
                self.get_connecting_block(Coord((r, c)), &mut region, &mut processed);

                regions.push(region);
            }
        }

        regions
    }

    fn get_connecting_block(
        &self,
        repr: Coord,
        region: &mut Vec<Coord>,
        processed: &mut Vec<Vec<bool>>,
    ) {
        region.push(repr);
        processed[repr.0 .0][repr.0 .1] = true;

        for Coord((r, c)) in self.get_neighbors(repr) {
            if processed[r][c] || self.map[r][c] != self[repr] {
                continue;
            }

            self.get_connecting_block(Coord((r, c)), region, processed);
        }
    }

    fn calc_peri(&self, region: &[Coord]) -> usize {
        let mut region_new = HashSet::new();

        let mut perimeter = 0;

        for block in region {
            perimeter += 4;
            for n in self.get_neighbors(*block) {
                if region_new.contains(&n) {
                    perimeter -= 2;
                }
            }
            region_new.insert(*block);
        }

        perimeter
    }

    fn calc_sides(&self, region: &[Coord]) -> usize {
        let mut side_x: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut side_y: HashMap<usize, Vec<usize>> = HashMap::new();

        for block in region {
            let [top, bottom, left, right] = self.get_sides(block);

            side_x
                .entry(bottom.0)
                .and_modify(|v| v.push(bottom.1))
                .or_insert_with(|| vec![bottom.1]);

            side_x
                .entry(top.0)
                .and_modify(|v| v.push(top.1))
                .or_insert_with(|| vec![top.1]);

            side_y
                .entry(left.1)
                .and_modify(|v| v.push(left.0))
                .or_insert_with(|| vec![left.0]);

            side_y
                .entry(right.1)
                .and_modify(|v| v.push(right.0))
                .or_insert_with(|| vec![right.0]);
        }

        let x_sides = side_x.iter_mut().fold(0, |mut acc, (row, start_poses)| {
            start_poses.sort();

            let mut prev = vec![];

            for &cur in start_poses.iter() {
                if prev.is_empty() {
                    prev.push(cur);
                    continue;
                }

                let last = *prev.last().unwrap();

                if cur == last {
                    prev.pop();
                    continue;
                }

                if cur - last > 1 && !prev.is_empty() || {
                    if let Some(yconnector) = side_y.get(&cur) {
                        // yconnector must have 2 {row} values or 2 {row - 1}
                        !(yconnector.iter().filter(|&&y| y == *row).count() == 2
                            || yconnector.iter().filter(|&&y| y == row - 1).count() == 2)
                    } else {
                        true
                    }
                } {
                    acc += 1;
                    prev.clear();
                }

                prev.push(cur);
            }

            if !prev.is_empty() {
                acc += 1;
            }

            acc
        });

        let y_sides = side_y.iter_mut().fold(0, |mut acc, (col, start_poses)| {
            start_poses.sort();

            let mut prev = vec![];

            for &cur in start_poses.iter() {
                if prev.is_empty() {
                    prev.push(cur);
                    continue;
                }

                let last = *prev.last().unwrap();

                if cur == last {
                    prev.pop();
                    continue;
                }

                if cur - last > 1 && !prev.is_empty() || {
                    if let Some(xconnector) = side_x.get(&cur) {
                        // yconnector must have 2 {row} values or 2 {row - 1}
                        !(xconnector.iter().filter(|&&x| x == *col).count() == 2
                            || xconnector.iter().filter(|&&x| x == col - 1).count() == 2)
                    } else {
                        true
                    }
                } {
                    acc += 1;
                    prev.clear();
                }

                prev.push(cur);
            }

            if !prev.is_empty() {
                acc += 1;
            }

            acc
        });

        x_sides + y_sides
    }

    fn get_sides(&self, pos: &Coord) -> [(usize, usize); 4] {
        let Coord((r, c)) = *pos;

        let top = (r, c);
        let bottom = (r + 1, c);
        let left = (r, c);
        let right = (r, c + 1);

        [top, bottom, left, right]
    }

    fn calc_area(&self, region: &[Coord]) -> usize {
        region.len()
    }
}

fn main() {
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    let input = &read_input_v1(12);

    let p = Input::parse(input);

    let regions = p.get_regions();

    let cost = regions.iter().fold(0, |acc, region| {
        let area = p.calc_area(region);
        let peri = p.calc_sides(region);
        println!("area: {}, peri: {}", area, peri);
        acc + area * peri
    });

    println!("{}", cost);
}
