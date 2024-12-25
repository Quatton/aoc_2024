use aoc_2024::read_input_v1;

#[derive(Debug)]
enum Schema {
    Key([i32; 5]),
    Lock([i32; 5]),
}

impl Schema {
    fn from_str(input: &str) -> Schema {
        let is_lock = input.starts_with("#####");
        let c = input.lines().skip(1).take(5).enumerate().fold(
            [if is_lock { 0 } else { 5 }; 5],
            |mut acc, (ln, cur)| {
                for (i, ch) in cur.chars().enumerate() {
                    if is_lock && ch == '#' {
                        acc[i] = (ln + 1) as i32;
                    }

                    if !is_lock && ch == '.' {
                        acc[i] = 4 - ln as i32;
                    }
                }
                acc
            },
        );

        if is_lock {
            Schema::Lock(c)
        } else {
            Schema::Key(c)
        }
    }

    fn is_key(&self) -> bool {
        match self {
            Schema::Key(_) => true,
            _ => false,
        }
    }

    fn fit(&self, other: &Schema) -> bool {
        let (key, lock) = if self.is_key() {
            if other.is_key() {
                return false;
            }
            (self, other)
        } else {
            (other, self)
        };

        match (key, lock) {
            (Schema::Key(k), Schema::Lock(l)) => {
                for i in 0..5 {
                    if k[i] + l[i] > 5 {
                        return false;
                    }
                }
                true
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    let input = read_input_v1(25);
    let schemas = input
        .split("\n\n")
        .map(Schema::from_str)
        .collect::<Vec<_>>();

    // println!("{:?}", schemas);

    let keys = schemas.iter().filter(|s| s.is_key()).collect::<Vec<_>>();
    let locks = schemas.iter().filter(|s| !s.is_key()).collect::<Vec<_>>();

    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.fit(lock) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
