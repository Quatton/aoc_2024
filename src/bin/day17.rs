use aoc_2024::read_input_v1;

struct Sim {
    pc: usize,
    program: Vec<usize>,
    out: Vec<u64>,
    reg: [u64; 3],
}

impl Sim {
    fn load(input: &str) -> Self {
        let (reg, program) = input.split_once("Program: ").unwrap();
        let regs = reg
            .trim()
            .lines()
            .map(|l| l.split_once(":").unwrap().1.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Sim {
            program: program
                .trim()
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
            pc: 0,
            reg: [regs[0], regs[1], regs[2]],
            out: vec![],
        }
    }

    fn run_once(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        // println!("reg: {:?}, pc: {}, out: {:?}", self.reg, self.pc, self.out);

        let ops = (self.program[self.pc], self.program[self.pc + 1]);

        self.execute(ops);
        false
    }

    fn init(&mut self) {
        self.pc = 0;
        self.out = vec![];
    }

    fn run(&mut self) {
        self.init();
        loop {
            if self.run_once() {
                break;
            }
        }
    }

    fn combo(&self, op: usize) -> u64 {
        match op {
            0..=3 => op as u64,
            4..=6 => self.reg[op - 4],
            _ => u64::MAX,
        }
    }

    fn execute(&mut self, op: (usize, usize)) {
        let c = self.combo(op.1);
        match op.0 {
            0 => self.reg[0] /= 2u64.pow(c as u32),
            1 => self.reg[1] ^= op.1 as u64,
            2 => self.reg[1] = c % 8,
            3 => {
                if self.reg[0] != 0 {
                    self.pc = op.1;
                    return;
                }
            }
            4 => self.reg[1] ^= self.reg[2],
            5 => self.out.push(c % 8),
            6 => self.reg[1] = self.reg[0] / (2u64.pow(c as u32)),
            7 => self.reg[2] = self.reg[0] / (2u64.pow(c as u32)),

            _ => unimplemented!(),
        }

        self.pc += 2
    }

    fn unprogram(&self, out: &[u64]) -> u64 {
        let mut vals = vec![0];
        for &o in out.iter().rev() {
            let mut new_vals = vec![];
            for val in vals {
                let checklist = (0..=7)
                    .skip((val == 0) as usize)
                    .map(|v| (v, (v ^ 2 ^ ((val * 8 + v) >> (v ^ 2)) ^ 7) & 7))
                    .collect::<Vec<_>>();

                for &(v, check) in checklist.iter() {
                    if check == o {
                        new_vals.push((val * 8) + v);
                    }
                }
            }

            vals = new_vals;
        }

        vals[0]
    }
}

fn main() {
    // let input = "Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0";
    let input = &read_input_v1(17);

    let mut sim = Sim::load(input);
    sim.run();

    let out = [2, 4, 1, 2, 7, 5, 4, 3, 0, 3, 1, 7, 5, 5, 3, 0];

    // 4 7

    println!("{:?}", &sim.out);
    let a = sim.unprogram(&out);
    println!("{:?}", a);

    // for a in 1..1000 {
    //     sim.reg = [a, 0, 0];
    //     sim.run();
    //     // println!("{a}: {:?}", sim.out);
    //     let copied = sim.out.clone();
    //     let guess = sim.unprogram(&sim.out);
    //     sim.reg = [guess, 0, 0];
    //     sim.run();
    //     if copied != sim.out {
    //         println!("mismatch occur at {a} != {guess} {copied:?}, {:?}", sim.out);
    //     }
    // }
}

// 2,4,
// b = a % 8

// 1,2
// b <= (a % 8) ^ 2

// 7,5
// c <= (a >> (a % 8) ^ 2)

// 4,3
// b <= (a % 8) ^ 2 ^ (a >> (a % 8) ^ 2)

// 0,3

// 1,7

// 5,5

// 3,0
