// X_A a + X_B b = X_t
// Y_A a + Y_B b = Y_t

use aoc_2024::read_input_v1;
use nom::{
    bytes::complete::tag,
    character::{
        complete::{digit1, multispace0},
        streaming::multispace1,
    },
    combinator::map,
    multi::many1,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
struct Problem {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    tx: f64,
    ty: f64,
}

fn parse_f64(input: &str) -> IResult<&str, f64> {
    map(digit1, |s: &str| s.parse::<f64>().unwrap())(input)
}

impl Problem {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, ax) = delimited(tag("Button A: X+"), parse_f64, tag(", Y+"))(input)?;
        let (input, ay) = terminated(parse_f64, multispace1)(input)?;
        let (input, bx) = delimited(tag("Button B: X+"), parse_f64, tag(", Y+"))(input)?;
        let (input, by) = terminated(parse_f64, multispace1)(input)?;
        let (input, tx) = delimited(tag("Prize: X="), parse_f64, tag(", Y="))(input)?;
        let (input, ty) = terminated(parse_f64, multispace0)(input)?;

        Ok((
            input,
            Self {
                ax,
                ay,
                bx,
                by,
                tx,
                ty,
            },
        ))
    }

    fn recalibrate(&mut self) {
        self.tx += 10000000000000.0;
        self.ty += 10000000000000.0;
    }

    fn solve(&self) -> (f64, f64) {
        let d = self.ax * self.by - self.ay * self.bx;

        if d == 0.0 {
            return (0.0, 0.0);
        }

        let x = (self.by * self.tx - self.bx * self.ty) / d;

        let y = (self.ax * self.ty - self.ay * self.tx) / d;

        (x, y)
    }

    fn cost(&self) -> usize {
        let (a, b) = self.solve();

        if a < 0.0 || b < 0.0 {
            return 0;
        }

        let tolerance = 1e-6;

        if (a.round() * self.ax + b.round() * self.bx - self.tx).abs() > tolerance
            || (a.round() * self.ay + b.round() * self.by - self.ty).abs() > tolerance
            || a.fract() > tolerance
            || b.fract() > tolerance
        {
            return 0;
        }

        let a = a as usize;
        let b = b as usize;

        3 * a + b
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    let input = read_input_v1(13);
    let (_, mut problems) = many1(Problem::parse)(&input).unwrap();

    let cost = problems
        .iter_mut()
        .map(|p| {
            p.recalibrate();
            p.cost()
        })
        .sum::<usize>();

    println!("Total cost: {}", cost);

    Ok(())
}
