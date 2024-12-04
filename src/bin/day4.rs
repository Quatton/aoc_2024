use std::fmt::Debug;

use aoc_2024::read_input_v1;

struct Puzzle {
    nrow: usize,
    ncol: usize,
    data: Vec<Vec<char>>,
}

impl Puzzle {
    fn match_for(&self, pattern: &str, from: (usize, usize), direction: (isize, isize)) -> bool {
        if pattern.is_empty() {
            return true;
        }

        let (row, col) = from;
        let (dr, dc) = direction;
        let (c, rest) = pattern.split_at(1);

        if row >= self.nrow || col >= self.nrow {
            return false;
        }

        if self.data[row][col] == c.chars().next().unwrap() {
            if rest.is_empty() {
                return true;
            }

            let (nr, ovfr) = row.overflowing_add_signed(dr);
            let (nc, ovfc) = col.overflowing_add_signed(dc);

            if ovfr || ovfc {
                return false;
            }

            return self.match_for(rest, (nr, nc), direction);
        }

        false
    }

    fn count_match(&self, pattern: &str) -> usize {
        let mut count = 0;
        for row in 0..self.nrow {
            for col in 0..self.ncol {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        if self.match_for(pattern, (row, col), (dr, dc)) {
                            // println!("{:?}: {:?}", (row, col), (dr, dc));
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn count_x(&self) -> usize {
        let mut count = 0;
        for row in 0..self.nrow {
            for col in 0..self.ncol {
                if (self.match_for("AM", (row, col), (-1, -1))
                    && self.match_for("AS", (row, col), (1, 1))
                    || self.match_for("AS", (row, col), (-1, -1))
                        && self.match_for("AM", (row, col), (1, 1)))
                    && (self.match_for("AS", (row, col), (-1, 1))
                        && self.match_for("AM", (row, col), (1, -1))
                        || self.match_for("AM", (row, col), (-1, 1))
                            && self.match_for("AS", (row, col), (1, -1)))
                {
                    println!("{:?}", (row, col));
                    count += 1
                }
            }
        }

        count
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            for c in row {
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_puzzle(input: &str) -> Puzzle {
    let rows = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Puzzle {
        nrow: rows[0].len(),
        ncol: rows.len(),
        data: rows,
    }
}

fn main() {
    let input = read_input_v1(4);
    let pz = parse_puzzle(&input);
    let count = pz.count_match("XMAS");
    println!("{count}");

    let countx = pz.count_x();
    println!("{countx}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM  
MXMXAXMASX";

        let pz = parse_puzzle(input);
        println!("{:?}", pz);

        println!("{:?}", pz.match_for("XMAS", (0, 4), (1, 1)));
        println!("{:?}", pz.count_match("XMAS"))
    }

    #[test]
    fn sample2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM  
MXMXAXMASX";

        let pz = parse_puzzle(input);
        println!("{:?}", pz);

        println!("{:?}", pz.count_x())
    }

    #[test]
    fn try_running() {
        println!("{:?}", [-1, 1].iter().zip([-1, 1]).collect::<Vec<_>>())
    }
}
