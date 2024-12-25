use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use aoc_2024::read_input_v1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum K {
    Z(usize),
    P(usize),
    X(usize),
    Y(usize),
    C(usize),
    A(usize),
    AC(usize),
}

#[derive(Debug, Clone, Copy)]
enum IdentifyError {
    Expect((usize, K, K)),
    NotIdentified(usize),
}

impl Display for IdentifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifyError::Expect((p, k1, k2)) => {
                write!(f, "expecting {} to be {} but got {}", decode(*p), k1, k2)
            }
            IdentifyError::NotIdentified(p) => write!(f, "{} not identified", decode(*p)),
        }
    }
}

impl Display for K {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            K::P(u) => write!(f, "p{:02}", u),
            K::Z(u) => write!(f, "z{:02}", u),
            K::X(u) => write!(f, "x{:02}", u),
            K::Y(u) => write!(f, "y{:02}", u),
            K::C(u) => write!(f, "c{:02}", u),
            K::A(u) => write!(f, "a{:02}", u),
            K::AC(u) => write!(f, "ac{:02}", u),
        }
    }
}

#[derive(Debug, Default)]
struct Input {
    reg: HashMap<usize, bool>,
    program: Vec<(Op, usize, usize, usize)>,
    lookup: HashMap<usize, (Op, usize, usize, usize)>,
    rename_table: HashMap<usize, K>,
    rename_table_lookup: HashMap<K, usize>,
    correction: HashMap<usize, K>,
    queue: VecDeque<usize>,
    pending: HashMap<usize, Vec<usize>>,
    bits: usize,
}

fn encode(z: &str) -> usize {
    z.chars().fold(0, |acc, cur| (acc << 8) + cur as usize)
}

fn decode(mut z: usize) -> String {
    let mut s = String::new();

    while z > 0 {
        s.insert(0, (z & 0xff) as u8 as char);
        z >>= 8;
    }

    s
}

#[derive(Debug)]
enum ProgramError {
    NE(usize),
    DP(usize),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    XOR,
    OR,
    AND,
}

impl Op {
    fn from_str(input: &str) -> Self {
        match input {
            "XOR" => Self::XOR,
            "OR" => Self::OR,
            "AND" => Self::AND,
            _ => unreachable!(),
        }
    }
}

impl Input {
    fn from_input(input: &str) -> Self {
        let (pre, pro) = input.split_once("\n\n").unwrap();
        let mut reg: HashMap<usize, bool> = HashMap::new();

        let mut bits = 0;
        for line in pre.lines() {
            let (k, v) = line.split_once(": ").unwrap();
            let ki = encode(k);
            let vi = v != "0";
            reg.insert(ki, vi);
            bits += 1;
        }

        let program: Vec<(Op, usize, usize, usize)> = pro
            .lines()
            .map(|l| {
                let token = l.split_whitespace().collect::<Vec<_>>();
                let rs1 = encode(token[0]);
                let op = Op::from_str(token[1]);
                let rs2 = encode(token[2]);
                let rd = encode(token[4]);
                (op, rs1, rs2, rd)
            })
            .collect();

        let lookup = program
            .iter()
            .map(|&(op, rs1, rs2, rd)| (rd, (op, rs1, rs2, rd)))
            .collect();

        Self {
            bits: bits >> 1,
            reg,
            queue: VecDeque::from_iter(0..program.len()),
            program,
            lookup,
            ..Default::default()
        }
    }

    // fn correct(&mut self, bit: usize) {
    //     let z = format!("z{:02}", bit);
    //     let zk = encode(&z);
    //     self.correct_helper(zk);
    // }

    // fn correct_helper(&mut self, rd: usize) -> K {
    //     let p = self.lookup.get(&rd).unwrap();
    // }

    fn trace(&self, bit: usize) {
        let z = format!("z{:02}", bit);
        let zk = encode(&z);
        let res = self.trace_helper(zk);
        for &(op, rs1, rs2, rd) in res.iter().rev() {
            println!(
                "{} = {} {} {}",
                self.rename_table
                    .get(&rd)
                    .map_or_else(|| decode(rd), |x| x.to_string()),
                self.rename_table
                    .get(&rs1)
                    .map_or_else(|| decode(rs1), |x| x.to_string()),
                match op {
                    Op::XOR => '^',
                    Op::AND => '&',
                    Op::OR => '|',
                },
                self.rename_table
                    .get(&rs2)
                    .map_or_else(|| decode(rs2), |x| x.to_string())
            );
        }
    }

    fn run(&mut self) -> usize {
        self.queue = VecDeque::from_iter(0..self.program.len());
        while let Some(line) = self.queue.pop_front() {
            self.run_program(line);
        }

        let z = (0..)
            .try_fold((0, 0), |acc, cur| {
                let zk = format!("z{:02}", cur);
                if let Some(&zv) = self.reg.get(&(encode(&zk))) {
                    let zv = zv as usize;
                    println!("{}: {}", zk, zv);
                    Ok((acc.0 + (zv << acc.1), acc.1 + 1))
                } else {
                    Err(acc)
                }
            })
            .unwrap_err()
            .0;

        z
    }

    fn rename(&mut self) {
        self.queue = VecDeque::from_iter(0..self.program.len());
        while let Some(line) = self.queue.pop_front() {
            self.rename_program(line);
        }
    }

    fn rename_program(&mut self, line: usize) {
        let p = self.program.get(line).unwrap();

        if let (Some(p1), Some(p2)) = (self.get_reg_name(p.1), self.get_reg_name(p.2)) {
            if let Some(res) = match (p1, p2) {
                (K::X(u), K::Y(_)) => Some(match p.0 {
                    Op::XOR => K::P(u),
                    Op::AND => K::A(u),
                    j => unimplemented!("not supported {j:?}"),
                }),
                (K::Y(u), K::X(_)) => Some(match p.0 {
                    Op::XOR => K::P(u),
                    Op::AND => K::A(u),
                    j => unimplemented!("not supported {j:?}"),
                }),
                _ => None,
            } {
                self.rename_table.insert(p.3, res);
                self.rename_table_lookup.insert(res, p.3);
            }
        }
    }

    fn get_reg_name(&self, key: usize) -> Option<K> {
        let num1 = (key >> 8 & 0xff) as u8 as char;
        let num2 = (key & 0xff) as u8 as char;
        let num = format!("{}{}", num1, num2).parse::<usize>();
        match ((key >> 16) & 0xff) as u8 as char {
            'x' => Some(K::X(num.unwrap())),
            'y' => Some(K::Y(num.unwrap())),
            _ => self.rename_table.get(&key).copied(),
        }
    }

    fn get_number(&self, key: usize) -> Result<usize, std::num::ParseIntError> {
        let num1 = (key >> 8 & 0xff) as u8 as char;
        let num2 = (key & 0xff) as u8 as char;
        let num = format!("{}{}", num1, num2).parse::<usize>();
        num
    }

    fn load_number(&mut self, bits: usize, mut x: usize, mut y: usize) {
        self.reg.clear();
        for i in 0..bits {
            let kx = encode(&format!("x{:02}", i));
            let ky = encode(&format!("y{:02}", i));

            self.reg.insert(kx, x & 1 == 1);
            self.reg.insert(ky, y & 1 == 1);
            x >>= 1;
            y >>= 1;
        }
    }

    fn test_bits(&mut self, bits: usize) {
        for max_bit in 1..=bits {
            let lb = (1 << (max_bit - 1)) - 1;
            let ub = 1 << max_bit;
            for x in lb..ub {
                for y in lb..ub {
                    self.load_number(bits, x, y);
                    let z = self.run();
                    if z != x + y {
                        self.trace(max_bit);
                        panic!(
                            "addition failed: {} + {} = {} =? {}, at bit {}",
                            x,
                            y,
                            z,
                            x + y,
                            max_bit
                        );
                    }
                }
            }
            println!("addition passed for bit {}", max_bit);
        }
    }

    fn identify(&mut self, reg: usize) -> Result<K, IdentifyError> {
        let renamed = self.get_reg_name(reg);

        if let Some(renamed) = renamed {
            return Ok(renamed);
        }

        let p = *self.lookup.get(&reg).unwrap();

        let res = self.identify_helper(reg, false);
        if res.is_err() {
            println!(
                "{}: {:?} {:?} {:?} -> ???",
                decode(reg),
                decode(p.1),
                p.0,
                decode(p.2),
            );
            return res;
        }

        let res = res.unwrap();

        println!(
            "{}: {:?} {:?} {:?} -> {}",
            decode(reg),
            decode(p.1),
            p.0,
            decode(p.2),
            res
        );

        self.rename_table.insert(reg, res);

        Ok(res)
    }

    fn identify_helper(&mut self, reg: usize, swapped: bool) -> Result<K, IdentifyError> {
        let p = *self.lookup.get(&reg).unwrap();

        let (p1, p2) = if swapped { (p.2, p.1) } else { (p.1, p.2) };

        let p1i = self.identify(p1)?;
        let p2i = self.identify(p2)?;

        let attempt1 = match p.0 {
            Op::XOR => match p1i {
                K::X(u) => Some(if u == 0 { K::Z(u) } else { K::P(u) }),
                K::P(u) => Some(K::Z(u)),
                _ => None,
            },
            Op::AND => match (p1i, p2i) {
                (K::Y(u), K::X(_)) => Some(if u == 0 { K::C(u) } else { K::A(u) }),
                (K::P(u), K::C(v)) => {
                    if u == v + 1 {
                        Some(K::AC(u))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Op::OR => match (p1i, p2i) {
                (K::AC(u), K::C(_)) => Some(K::C(u)),
                (K::AC(u), K::A(_)) => Some(K::C(u)),
                _ => None,
            },
        };

        if let Some(attempt1) = attempt1 {
            return Ok(attempt1);
        }

        if swapped {
            return Err(IdentifyError::NotIdentified(reg));
        }

        self.identify_helper(p.3, true)
    }

    fn trace_helper(&self, rd: usize) -> Vec<(Op, usize, usize, usize)> {
        let p = self.lookup.get(&rd).unwrap_or_else(|| {
            panic!("{} not found", decode(rd));
        });
        let mut res = vec![*p];
        if !matches!(((p.1 >> 16) & 0xff) as u8 as char, 'x' | 'y') {
            res.extend(self.trace_helper(p.1));
            res.extend(self.trace_helper(p.2));
        }

        res
    }

    fn run_program(&mut self, line: usize) {
        let p = self.program.get(line).unwrap();
        let res = match (self.reg.get(&p.1), self.reg.get(&p.2)) {
            (Some(&rs1), Some(&rs2)) => {
                self.pending.remove(&p.2);
                self.pending.remove(&p.1);

                Some(match p.0 {
                    Op::XOR => rs1 ^ rs2,
                    Op::AND => rs1 & rs2,
                    Op::OR => rs1 | rs2,
                })
            }
            (Some(_), None) => {
                self.pending.entry(p.2).or_default().push(line);
                None
            }
            (None, Some(_)) => {
                self.pending.entry(p.1).or_default().push(line);

                None
            }
            _ => {
                self.pending.entry(p.2).or_default().push(line);
                self.pending.entry(p.1).or_default().push(line);

                None
            }
        };

        if let Some(res) = res {
            self.reg.insert(p.3, res);
            if let Some(pendings) = self.pending.get(&p.3) {
                for &p in pendings.iter().rev() {
                    self.queue.push_front(p);
                }
            }
        }
    }
}

fn main() {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    let input = &read_input_v1(24);
    let mut p = Input::from_input(input);

    for i in 0..46 {
        let z = p.identify(encode(&format!("z{:02}", i)));
        if z.is_err() {
            println!("{}: {}", i, z.unwrap_err());
            break;
        }
    }
}
