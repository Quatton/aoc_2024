use aoc_2024::read_input_v1;

fn check(diff: &i32) -> bool {
    (1..=3).contains(diff)
}

fn slice_without_index(vec: &[i32], i: usize) -> Vec<i32> {
    let mut vec = vec.to_owned();
    vec.remove(i);
    vec
}

fn check_level_helper(vec: &[i32]) -> (usize, bool) {
    let mut iter = vec.iter().enumerate();
    let (_, prev) = iter.next().unwrap();
    let mut prev = *prev;
    let mut dir = 0;

    for (i, &curr) in iter {
        let mut diff = curr - prev;
        if dir == 0 {
            dir = diff.signum();
        }
        diff *= dir;

        if !check(&diff) {
            return (i, false);
        }

        prev = curr;
    }

    (vec.len(), true)
}

fn check_level(vec: &[i32]) -> bool {
    let (i, res) = check_level_helper(vec);

    if res {
        return true;
    }

    for p in i.saturating_sub(2)..=i {
        let (_, res) = check_level_helper(&slice_without_index(vec, p));
        if res {
            return true;
        }
    }

    false
}

fn part2(input: &str) -> usize {
    let levelss = input.lines().map(|p| {
        p.split_whitespace()
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut safe = 0;

    for levels in levelss {
        if check_level(&levels) {
            safe += 1;
        }
    }

    safe
}

fn part1(input: &str, max_tolerance: usize) -> usize {
    let levelss = input.lines().map(|p| {
        p.split_whitespace()
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut safe = 0;

    for levels in levelss {
        let mut error = 0;
        safe += 1;
        let mut ci = 0;
        let mut dir = 0;
        let mut pi = 0;
        while ci + 1 < levels.len() {
            ci += 1;
            let cur = levels[ci];
            let prev = levels[pi];

            let mut diff = cur - prev;

            // reset dir?
            if pi == 0 || ci == 2 && error == 1 {
                dir = diff.signum();
            }

            diff *= dir;

            if check(&diff) {
                pi = ci;
                continue;
            }

            // println!("{levels:?}: {pi} {ci}");

            // 1 3 2 4 5
            //   ^ ^

            // let's first check error
            if error >= max_tolerance {
                safe -= 1;
                break;
            }

            // if it's at the end then don't care
            if ci == levels.len() - 1 {
                break;
            }

            // remove either pi or ci
            let mut tmp_dir = dir;
            let mut ok = false;

            // case 1: remove pi
            let next = levels[pi + 1]; // can confirm no out of bound because it'd've bailed out
            let next_diff = next - cur;

            // when does we change direction after removing pi?
            // when pi = 0, ci = 1 and remove pi now cur is now the head
            // when pi = 1, ci = 2 and remove pi now cur and pi 0 might have different opinion

            let diff = if ci <= 2 {
                tmp_dir = next_diff.signum();
                next_diff.abs()
            } else {
                next_diff * tmp_dir
            };

            if check(&diff) {
                if pi > 0 {
                    // need to check that diff with pi - 1 works
                    let prev = levels[pi - 1];
                    let diff = (cur - prev) * tmp_dir;

                    if check(&diff) {
                        ok = true;
                        dir = tmp_dir;
                    }
                } else {
                    dir = tmp_dir;
                    ok = true;
                }
            }

            if ok {
                pi = ci;
            }

            // if we reached here means must remove ci
            // we will check that in the next cycle anyway
            error += 1
        }
    }

    safe
}
fn main() {
    let input = &read_input_v1(2);

    // println!("\n{}", part1(input, 1))
    println!("\n{}", part2(input))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn sample() {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(4, part1(input, 1))
    }

    #[test]
    fn sampl2() {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(4, part2(input))
    }

    #[test]
    fn repeated() {
        assert_eq!(1, part1("1 1 2 3 4", 1))
    }

    #[test]
    fn repeated_too_much() {
        assert_eq!(0, part1("1 1 1 3 4", 1))
    }

    #[test]
    fn test() {
        let org = [1, 1, 2, 3, 4];
        let all = (0..org.len())
            .map(|i| slice_without_index(&org, i))
            .collect::<Vec<_>>();

        println!("{all:?}");
    }
}
