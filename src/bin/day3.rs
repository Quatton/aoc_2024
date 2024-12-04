use aoc_2024::read_input_v1;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::map,
    multi::many0,
    IResult,
};

#[derive(Debug)]
enum Action {
    Mul(i32, i32),
    Do,
    Dont,
}

fn main() {
    let input = read_input_v1(3);
    println!("{}", muls(&input));
}

fn mul(input: &str) -> IResult<&str, Action> {
    let (input, _) = tag("mul(")(input)?;
    let (input, i1) = map(digit1, |s: &str| s.parse::<i32>().unwrap())(input)?;
    let (input, _) = char(',')(input)?;
    let (input, i2) = map(digit1, |s: &str| s.parse::<i32>().unwrap())(input)?;
    let (input, _) = char(')')(input)?;

    Ok((input, Action::Mul(i1, i2)))
}

fn do_parse(input: &str) -> IResult<&str, Action> {
    alt((
        map(tag("do()"), |_| Action::Do),
        map(tag("don't()"), |_| Action::Dont),
    ))(input)
}

fn muls_helper(mut input: &str, mut vec: Vec<Action>) -> IResult<&str, Vec<Action>> {
    while !input.is_empty() {
        let (s, muls) = many0(alt((mul, do_parse)))(input)?;
        vec.extend(muls);
        let (s, _) = anychar(s)?;
        input = s
    }

    Ok((input, vec))
}

fn muls(input: &str) -> i32 {
    let (_, set) = muls_helper(input, vec![]).unwrap();

    let mut res = 0;
    let mut enabled = true;

    for action in set {
        match action {
            Action::Do => enabled = true,
            Action::Dont => enabled = false,
            Action::Mul(i1, i2) => {
                if enabled {
                    res += i1 * i2
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (_, set) = muls_helper(input, vec![]).unwrap();
        println!("{:?}", set);
    }
}
