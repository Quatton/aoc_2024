use std::fmt::Debug;

use aoc_2024::read_input_v1;

struct Input {
    blocks: Vec<Option<usize>>,
    bws: Vec<(usize, Option<usize>)>,
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for block in self.blocks.iter() {
        //     match block {
        //         Some(id) => write!(f, "{id}")?,
        //         None => write!(f, ".")?,
        //     }
        // }
        // writeln!(f)?;

        for &(size, block) in self.bws.iter() {
            for _ in 0..size {
                match block {
                    Some(id) => write!(f, "{id}")?,
                    None => write!(f, ".")?,
                }
            }
        }

        Ok(())
    }
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut curr = 0;
        let mut is_empty = false;

        let mut blocks = vec![];
        let mut bws = vec![];

        for block in input.chars() {
            let size = block.to_digit(10).unwrap();

            for _ in 0..size {
                if is_empty {
                    blocks.push(None)
                } else {
                    blocks.push(Some(curr))
                }
            }

            if !is_empty {
                is_empty = true;
                bws.push((size as usize, Some(curr)));
                curr += 1;
            } else {
                is_empty = false;
                bws.push((size as usize, None))
            }
        }

        Self { blocks, bws }
    }

    fn align(&mut self) {
        let mut left = 0;
        let mut right = self.blocks.len() - 1;

        while left < right {
            if self.blocks[left].is_some() {
                left += 1;
            }

            if self.blocks[right].is_none() {
                right -= 1;
            }

            if self.blocks[left].is_none() && self.blocks[right].is_some() {
                self.blocks.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
    }

    fn align_whole(&mut self) {
        let mut right = self.bws.len() - 1;
        while right > 0 {
            let (size, id) = self.bws[right];
            if let Some(id) = id {
                for i in 0..right {
                    if self.bws[i].1.is_none() && self.bws[i].0 >= size {
                        self.bws[i].0 -= size;
                        self.bws[right] = (size, None);
                        self.bws.insert(i, (size, Some(id)));
                        // println!("{self:?}");
                        break;
                    }
                }
            }

            right -= 1;
        }
    }

    fn checksum(&self) -> usize {
        self.blocks.iter().enumerate().fold(0, |acc, (idx, block)| {
            if let Some(block) = block {
                acc + idx * block
            } else {
                acc
            }
        })
    }

    fn checksum_whole(&self) -> usize {
        let mut idx = 0;
        let mut acc = 0;
        for &(size, block) in self.bws.iter() {
            for _ in 0..size {
                if let Some(block) = block {
                    acc += idx * block;
                }
                idx += 1;
            }
        }

        acc
    }
}

fn main() {
    // let input = "2333133121414131402";
    let input = &read_input_v1(9);
    let mut p = Input::parse(input);
    println!("{p:?}");
    p.align_whole();
    println!("{p:?}");
    println!("{}", p.checksum_whole());
}
