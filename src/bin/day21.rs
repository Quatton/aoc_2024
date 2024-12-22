#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Key {
    L,
    R,
    U,
    D,
    A,
}

impl Key {
    fn to_char(&self) -> char {
        match self {
            Key::L => '<',
            Key::R => '>',
            Key::U => '^',
            Key::D => 'v',
            Key::A => 'A',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '<' => Key::L,
            '>' => Key::R,
            '^' => Key::U,
            'v' => Key::D,
            'A' => Key::A,
            _ => panic!("invalid char"),
        }
    }
}

fn press_key(key: Key, pressed: Key) -> Key {
    match pressed {
        Key::A => key,
        Key::D => match key {
            Key::U => Key::D,
            Key::A => Key::R,
            _ => key,
        },
        Key::L => match key {
            Key::R => Key::D,
            Key::A => Key::U,
            Key::D => Key::L,
            _ => key,
        },
        Key::R => match key {
            Key::L => Key::D,
            Key::D => Key::R,
            Key::U => Key::A,
            _ => key,
        },
        Key::U => match key {
            Key::R => Key::A,
            Key::D => Key::U,
            _ => key,
        },
    }
}

fn press_num(num: i32, pressed: Key) -> i32 {
    match num {
        0 => match pressed {
            Key::U => 2,
            Key::R => 10,
            _ => 0,
        },
        1 => match pressed {
            Key::U => 4,
            Key::R => 2,
            _ => 1,
        },
        2 => match pressed {
            Key::U => 5,
            Key::R => 3,
            Key::L => 1,
            Key::D => 0,
            _ => 2,
        },
        3 => match pressed {
            Key::U => 6,
            Key::L => 2,
            Key::D => 10,
            _ => 3,
        },
        4 => match pressed {
            Key::R => 5,
            Key::D => 1,
            Key::U => 7,
            _ => 4,
        },
        5 => match pressed {
            Key::U => 8,
            Key::R => 6,
            Key::L => 4,
            Key::D => 2,
            _ => 5,
        },
        6 => match pressed {
            Key::U => 9,
            Key::L => 5,
            Key::D => 3,
            _ => 6,
        },
        7 => match pressed {
            Key::R => 8,
            Key::D => 4,
            _ => 7,
        },
        8 => match pressed {
            Key::R => 9,
            Key::L => 7,
            Key::D => 5,
            _ => 8,
        },
        9 => match pressed {
            Key::L => 8,
            Key::D => 6,
            _ => 9,
        },
        10 => match pressed {
            Key::L => 0,
            Key::U => 3,
            _ => 10,
        },
        _ => panic!("invalid num"),
    }
}

fn next_num(num: i32, target: i32) -> Option<Key> {
    if num == target {
        return None;
    }

    Some(match num {
        0 => match target {
            2 => Key::U,
            10 => Key::R,
            _ => Key::U,
        },
        1 => match target {
            0 | 2 | 3 | 10 => Key::R,
            _ => Key::U,
        },
        2 => match target {
            1 | 4 | 7 => Key::L,
            3 | 6 | 9 => Key::R,
            0 | 10 => Key::D,
            _ => Key::U,
        },
        3 => match target {
            10 => Key::D,
            4 | 6 | 9 => Key::U,
            _ => Key::L,
        },
        4 => match target {
            0..=3 | 10 => Key::D,
            5 | 6 | 8 | 9 => Key::R,
            _ => Key::U,
        },
        5 => match target {
            0..=3 | 10 => Key::D,
            4 => Key::L,
            6 => Key::R,
            _ => Key::U,
        },
        6 => match target {
            0..=3 | 10 => Key::D,
            4 | 5 => Key::L,
            _ => Key::U,
        },
        7 => match target {
            8 | 9 => Key::R,
            _ => Key::D,
        },
        8 => match target {
            7 => Key::L,
            9 => Key::R,
            _ => Key::D,
        },
        9 => match target {
            7 | 8 => Key::L,
            _ => Key::D,
        },
        10 => match target {
            0 => Key::L,
            _ => Key::U,
        },
        _ => panic!("invalid num"),
    })
}

fn next_keypad(key: Key, target: Key) -> Option<Key> {
    if key == target {
        return None;
    }

    Some(match key {
        Key::L => Key::R,
        Key::D => match target {
            Key::A => Key::R,
            _ => target,
        },
        Key::U => match target {
            Key::A => Key::R,
            _ => Key::D,
        },
        Key::R => match target {
            Key::A => Key::U,
            _ => Key::L,
        },
        Key::A => match target {
            Key::U | Key::D => Key::L,
            _ => Key::D,
        },
    })
}

fn join_keypad(mut key: Key, target: Key) -> String {
    let mut keys = String::new();

    // println!("key: {:?}, target: {:?}", key, target);
    while key != target {
        if let Some(pressed) = next_keypad(key, target) {
            keys.push(pressed.to_char());
            key = press_key(key, pressed);
            // println!("prrssed: {:?}, key: {:?}", pressed, key);
        }
    }

    keys
}

fn join_num(mut num: i32, target: i32) -> String {
    let mut keys = String::new();

    while num != target {
        if let Some(pressed) = next_num(num, target) {
            keys.push(pressed.to_char());
            // println!("pressed: {:?}, num: {}", pressed, num);
            num = press_num(num, pressed);
        }
    }

    keys
}

fn join_num_str(input: &str) -> String {
    let mut out = String::new();
    let mut cur = 10;

    for target in input.chars().map(|c| c.to_digit(11).unwrap() as i32) {
        out.push_str(&join_num(cur, target));
        out.push_str("A");
        cur = target;
    }

    out
}

fn join_keypad_str(input: &str) -> String {
    let mut out = String::new();
    let mut cur = Key::A;

    for target in input.chars().map(Key::from_char) {
        out.push_str(&join_keypad(cur, target));
        out.push_str("A");
        cur = target;
    }

    // println!("out: {}", out);
    out
}

fn calc_complexity(input: &str) -> usize {
    let inp: usize = input.trim_end_matches('A').parse().unwrap();
    let res = join_keypad_str(&join_keypad_str(&join_num_str(input)));

    println!("input: {}, res: {}", inp, res.len());
    inp * res.len()
}

fn simulate_keypad(input: &str) -> String {
    let mut out = String::new();
    let mut cur = Key::A;

    for pressed in input.chars().map(Key::from_char) {
        let next = press_key(cur, pressed);
        if pressed == Key::A {
            out.push(cur.to_char());
        }
        cur = next;
    }

    out
}

fn main() {
    let input = "029A
980A
179A
456A
379A";
    println!("{}", input.lines().map(calc_complexity).sum::<usize>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len(),
            join_keypad_str("<A^A>^^AvvvA").len()
        );

        assert_eq!(
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(),
            join_keypad_str(&join_keypad_str("<A^A>^^AvvvA")).len()
        )
    }
}
