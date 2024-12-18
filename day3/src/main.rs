use winnow::combinator::{repeat, seq};
use winnow::stream::AsChar;
use winnow::token::{any, take_till, take_while};
use winnow::{combinator::alt, prelude::*};

#[derive(Debug, Clone, Copy)]
enum Token {
    Mul(u16, u16),
    Do,
    Dont,
}

fn parse_token(i: &mut &str) -> PResult<Option<Token>> {
    fn number(i: &mut &str) -> PResult<u16> {
        take_while(1..=3, AsChar::is_dec_digit)
            .parse_to()
            .parse_next(i)
    }

    alt((
        seq!(_: "mul(", number, _: ",", number, _:")").map(|(a, b)| Some(Token::Mul(a, b))),
        "do()".value(Some(Token::Do)),
        "don't()".value(Some(Token::Dont)),
        take_till(1.., ('m', 'd')).value(None), // consume until next (possible) token
        any.value(None),                        // consume one character
    ))
    .parse_next(i)
}

fn part1(input: &str) -> u32 {
    // TODO: see if delimited(many(any), separated(many(any), parse_token), many(any)) works (probably not)

    let i: Vec<_> = repeat(1..2000, parse_token).parse(input).unwrap();
    i.iter()
        .filter_map(|x| {
            x.map(|t| {
                if let Token::Mul(a, b) = t {
                    a as u32 * b as u32
                } else {
                    0
                }
            })
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let i: Vec<_> = repeat(1..2000, parse_token).parse(input).unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for t in i.iter().filter_map(|&x| x /* lmao */) {
        match (t, enabled) {
            (Token::Do, _) => enabled = true,
            (Token::Dont, _) => enabled = false,
            (Token::Mul(a, b), true) => sum += a as u32 * b as u32,
            _ => {}
        }
    }
    sum
}

fn main() {
    let (p1_sample, p2_sample) = (
        part1(include_str!("sample.txt")),
        part2(include_str!("sample.txt")),
    );

    let (p1_answer, p2_answer) = (
        part1(include_str!("input.txt")),
        part2(include_str!("input.txt")),
    );

    if match std::env::var("PRINT_RESULT") {
        Ok(val) => val == "1",
        Err(_) => true,
    } {
        println!("Part 1 (sample): {p1_sample}",);
        println!("Part 1: {p1_answer}",);
        println!("\n{}\n", "=".repeat(30));
        println!("Part 2 (sample): {p2_sample}",);
        println!("Part 2: {p2_answer}",);
    }
}
