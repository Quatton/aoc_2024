use aoc_2024::read_input_v1;

fn check(diff: &i32) -> bool {
    (1..=3).contains(diff)
}

fn part1(input: &str, tolerance: usize) -> usize {
    let levelss = input.lines().map(|p| {
        p.split_whitespace()
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut safe = 0;

    for levels in levelss {
        let mut tolerance = tolerance;
        let mut i = 0;
        safe += 1;
        let mut direction = 0;

        let mut prev = levels[0];

        print!("\n{prev} ");

        let mut pprev = 0;
        while i < levels.len() - 1 {
            i += 1;
            let level = levels[i];
            let mut diff = level - prev;

            if direction == 0 {
                direction = diff.signum();
            }

            diff *= direction;

            if !check(&diff) {
                if tolerance == 0 {
                    print!("tolerance: {:?}", levels.get(..=i).unwrap());

                    safe -= 1;
                    break;
                } else {
                    tolerance -= 1;

                    if i + 1 >= levels.len() {
                        break;
                    }

                    let peek = levels[i + 1];
                    let diff_rm_level = (peek - prev) * direction;
                    let diff_rm_prev = peek - level;

                    if check(&diff_rm_level) {
                        continue;
                    }

                    if i < 2 {
                        if check(&diff_rm_prev.abs()) {
                            direction = diff_rm_prev.signum();
                            pprev = prev;
                            prev = level;
                            print!("{} ", prev);
                            continue;
                        }
                    } else {
                        let pprev_diff = (level - pprev) * direction;

                        if check(&(diff_rm_prev * direction)) && check(&pprev_diff) {
                            pprev = prev;
                            prev = level;
                            print!("{} ", prev);
                            continue;
                        }
                    }

                    print!(
                        "remove not help {}: ({:?})",
                        levels.get(i).unwrap(),
                        levels.get(i + 1)
                    );

                    safe -= 1;
                    break;
                }
            }

            pprev = prev;
            prev = level;
            print!("{} ", prev);
        }
    }

    safe
}

fn main() {
    let input = &read_input_v1(2);

    // let input = "1 1 2 3 4";

    println!("\n{}", part1(input, 1))
}
