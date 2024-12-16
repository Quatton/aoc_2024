use std::cmp::Ordering;

use aoc_2024::read_input_v1;

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

#[derive(Default, Debug, Clone)]
struct Map {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl Map {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    fn load_robots(mut self, input: &str) -> Self {
        self.robots = input.lines().map(Robot::from_str).collect();
        self
    }

    fn step(&mut self, nstep: i32) {
        for robot in self.robots.iter_mut() {
            robot.step(nstep, self.width, self.height);
        }
    }

    fn safety_factor(&self) -> i32 {
        let qx = self.width / 2;
        let qy = self.height / 2;

        let mut qs = vec![0, 0, 0, 0];

        for r in self.robots.iter() {
            match (r.p.0.cmp(&qx), r.p.1.cmp(&qy)) {
                (Ordering::Less, Ordering::Less) => qs[0] += 1,
                (Ordering::Less, Ordering::Greater) => qs[1] += 1,
                (Ordering::Greater, Ordering::Less) => qs[2] += 1,
                (Ordering::Greater, Ordering::Greater) => qs[3] += 1,
                _ => {}
            }
        }

        println!("{:?}", qs);

        qs.into_iter().filter(|&p| p > 0).product::<i32>()
    }

    fn print_map(&self) {
        let mut map = vec![vec!['.'; self.width as usize]; self.height as usize];

        for r in self.robots.iter() {
            map[r.p.1 as usize][r.p.0 as usize] = '#';
        }

        for row in map {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let (p, v) = s.split_once(" ").unwrap();
        let p = p.split_once('=').unwrap().1;
        let v = v.split_once('=').unwrap().1;
        let p = p
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let v = v
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Self {
            p: (p[0], p[1]),
            v: (v[0], v[1]),
        }
    }

    fn step(&mut self, nstep: i32, width: i32, height: i32) {
        self.p.0 = (self.p.0 + self.v.0 * nstep).rem_euclid(width);
        self.p.1 = (self.p.1 + self.v.1 * nstep).rem_euclid(height);
    }
}

fn main() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let mut map = Map::new(11, 7).load_robots(input);
    let input = &read_input_v1(14);
    let mut map = Map::new(101, 103).load_robots(input);

    // map.step(stepped);

    map.step(93);
    for i in 0..100 {
        map.step(101);
        println!("Step {}", i + 1);
        map.print_map();

        // take input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }

    println!("{}", map.safety_factor());
}
