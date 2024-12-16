use std::collections::HashSet;

use aoc_2024::read_input_v1;

struct Input {
    grid: Vec<Vec<char>>,
    moves: Vec<char>,
}

impl Input {
    fn from_str(s: &str) -> Self {
        let (grid, moves) = s.split_once("\n\n").unwrap();
        let grid = grid
            .trim()
            .lines()
            .map(|l| l.trim().chars().collect())
            .collect();

        Self {
            grid,
            moves: moves.replace(" ", "").replace("\n", "").chars().collect(),
        }
    }
}

enum Object {
    Box,
    Wall,
    Blank,
}

struct Map {
    robot: (i32, i32),
    boxes: HashSet<(usize, usize)>,
    walls: HashSet<(usize, usize)>,
    nrow: usize,
    ncol: usize,
    snapshot: String,
}

impl Map {
    fn print_grid(&self) -> String {
        let mut grid = String::new();
        for r in 0..self.nrow {
            for c in 0..self.ncol {
                if self.robot == (r as i32, c as i32) {
                    grid.push('@');
                } else if self.boxes.contains(&(r, c)) {
                    grid.push('O');
                } else if self.walls.contains(&(r, c)) {
                    grid.push('#');
                } else {
                    grid.push('.');
                }
            }
            grid.push('\n');
        }

        grid
    }

    fn print_grid_twice(&self) -> String {
        let mut grid = String::new();
        for r in 0..self.nrow {
            for c in 0..(self.ncol * 2) {
                if self.robot == (r as i32, c as i32) {
                    // push red @
                    grid.push_str("\x1b[0;31m@\x1b[0m");
                } else if self.walls.contains(&(r, c / 2 * 2)) {
                    grid.push('#');
                } else if self.boxes.contains(&(r, c)) {
                    grid.push('[');
                } else if c > 0 && self.boxes.contains(&(r, c - 1)) {
                    grid.push(']');
                } else {
                    grid.push('.');
                }
            }
            grid.push('\n');
        }

        grid
    }

    fn from_grid(i: Vec<Vec<char>>) -> Self {
        let mut robot = (0, 0);
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();

        for (r, row) in i.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                match col {
                    'O' => {
                        boxes.insert((r, c));
                    }
                    '@' => {
                        robot = (r as i32, c as i32);
                    }
                    '#' => {
                        walls.insert((r, c));
                    }
                    _ => {}
                };
            }
        }

        Self {
            ncol: i[0].len(),
            nrow: i.len(),
            robot,
            boxes,
            walls,
            snapshot: String::new(),
        }
    }

    fn from_grid_twice(i: Vec<Vec<char>>) -> Self {
        let mut robot = (0, 0);
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();

        for (r, row) in i.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                match col {
                    'O' => {
                        boxes.insert((r, c * 2));
                    }
                    '@' => {
                        robot = (r as i32, (c * 2) as i32);
                    }
                    '#' => {
                        walls.insert((r, c * 2));
                    }
                    _ => {}
                };
            }
        }

        let mut s = Self {
            ncol: i[0].len(),
            nrow: i.len(),
            robot,
            boxes,
            walls,
            snapshot: String::new(),
        };

        s.snapshot = s.print_grid_twice();

        s
    }

    fn step(&mut self, mv: char) {
        let (v0, v1) = match mv {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            c => unimplemented!("Unknown move: {}", c),
        };
        let r0 = self.robot.0 + v0;
        let r1 = self.robot.1 + v1;
        let r0u = r0 as usize;
        let r1u = r1 as usize;

        if self.walls.contains(&(r0u, r1u)) {
            return;
        }

        if self.boxes.contains(&(r0u, r1u)) && !self.try_move_box(&(r0u, r1u), &(v0, v1)) {
            return;
        }

        self.robot = (r0, r1);
    }

    fn step_twice(&mut self, mv: char) {
        let dir = match mv {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            c => unimplemented!("Unknown move: {}", c),
        };
        let (v0, v1) = dir;
        let r0 = self.robot.0 + v0;
        let r1 = self.robot.1 + v1;
        let r0u = r0 as usize;
        let r1u = r1 as usize;

        let cols = match dir.1 {
            // ##  ##
            // @    @
            0 => [r1u - 1, r1u].to_vec(),
            // @##
            1 => [r1u].to_vec(),
            // ##@
            -1 => [r1u - 1].to_vec(),
            _ => unreachable!(),
        };

        let mut rem_pos = HashSet::new();
        let mut new_pos = HashSet::new();

        for col in cols {
            if self.walls.contains(&(r0u, col)) {
                return;
            }

            if self.boxes.contains(&(r0u, col)) {
                if let Some((rem, new)) = self.try_move_box_twice(&(r0u, col), &dir) {
                    rem_pos.extend(rem);
                    new_pos.extend(new);
                } else {
                    return;
                }
            }
        }

        for pos in rem_pos {
            self.boxes.remove(&pos);
        }

        for pos in new_pos {
            self.boxes.insert(pos);
        }

        self.robot = (r0, r1);
        // self.snapshot = self.print_grid_twice();
    }

    fn try_move_box_twice(
        &mut self,
        pos: &(usize, usize),
        dir: &(i32, i32),
    ) -> Option<(HashSet<(usize, usize)>, HashSet<(usize, usize)>)> {
        let r0 = pos.0 as i32 + dir.0;
        let r1l = pos.1 as i32 + dir.1;
        let r0u = r0 as usize;
        let r1lu = r1l as usize;
        let mut rem_pos = HashSet::new();
        let mut new_pos = HashSet::new();

        let cols = match dir.1 {
            // ##  ##  ##
            // [] []    []
            0 => [r1lu + 1, r1lu - 1, r1lu].to_vec(),
            // []##
            1 => [r1lu + 1].to_vec(),
            // ##[]
            -1 => [r1lu - 1].to_vec(),
            _ => unreachable!(),
        };

        for col in cols {
            if self.walls.contains(&(r0u, col)) {
                return None;
            }

            if self.boxes.contains(&(r0u, col)) {
                let (rem, new) = self.try_move_box_twice(&(r0u, col), dir)?;
                rem_pos.extend(rem);
                new_pos.extend(new);
            }
        }

        rem_pos.insert(*pos);
        new_pos.insert((r0u, r1lu));

        Some((rem_pos, new_pos))
    }

    fn try_move_box(&mut self, pos: &(usize, usize), dir: &(i32, i32)) -> bool {
        let r0 = pos.0 as i32 + dir.0;
        let r1 = pos.1 as i32 + dir.1;
        let r0u = r0 as usize;
        let r1u = r1 as usize;

        if self.walls.contains(&(r0u, r1u)) {
            return false;
        }

        if self.boxes.contains(&(r0u, r1u)) && !self.try_move_box(&(r0u, r1u), dir) {
            return false;
        } else {
            self.boxes.remove(pos);
            self.boxes.insert((r0u, r1u));
        }

        true
    }

    fn gps(&self) -> i32 {
        self.boxes
            .iter()
            .fold(0, |acc, (r, c)| acc + (r * 100 + c) as i32)
    }
}

fn main() {
    let input = "##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    // let input = "
    // #######
    // #...#.#
    // #.....#
    // #.....#
    // #.OOO.#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######

    // <vv<<^^^";

    let input = &read_input_v1(15);
    let Input { grid, moves } = Input::from_str(input);
    let mut map = Map::from_grid_twice(grid);

    for mv in moves {
        // println!("{}", map.print_grid_twice());
        // println!("move: {}", mv);
        map.step_twice(mv);
    }

    // println!("{}", map.print_grid_twice());
    println!("{}", map.gps());
}
