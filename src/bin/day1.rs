use std::collections::BinaryHeap;

use aoc_2024::read_input_v1;

fn find_sum(left: &[usize], right: &[usize]) -> usize {
    let length = left.len();
    let mut lp = 0;
    let mut rp = 0;
    let mut sum = 0;

    while lp < length || rp < length {
        while left[lp] != right[rp] {
            if left[lp] > right[rp] {
                rp += 1;
                if rp == length {
                    return sum;
                }
            }

            if right[rp] > left[lp] {
                lp += 1;
                if lp == length {
                    return sum;
                }
            }
        }

        let cur = left[lp];
        let mut ln = 0;
        let mut rn = 0;

        while right[rp] == cur {
            rn += 1;
            rp += 1;
            if rp == length {
                break;
            }
        }

        while left[lp] == cur {
            ln += 1;
            lp += 1;
            if lp == length {
                break;
            }
        }

        sum += cur * ln * rn;
    }

    sum
}

fn main() {
    let input = read_input_v1(1);

    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    for line in input.lines() {
        let mut line = line.split_whitespace();
        let l = line.next().unwrap();
        let r = line.next().unwrap();
        left.push(l.parse::<usize>().unwrap());
        right.push(r.parse::<usize>().unwrap());
    }

    let left = left.into_sorted_vec();
    let right = right.into_sorted_vec();

    // part 1
    // let mut res = 0;

    // for (l, r) in left.into_iter().zip(right.into_iter()) {
    //     res += l.abs_diff(r)
    // }

    // println!("{res}");

    let sum = find_sum(&left, &right);

    println!("{sum}");
}
