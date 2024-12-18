use std::convert::Into;

use itertools::{izip, multiunzip, Itertools};
use winnow::ascii::{dec_uint, line_ending, space1};
use winnow::combinator::{opt, repeat, separated_pair, terminated};
use winnow::prelude::*;

#[derive(Debug)]
struct Line(u32, u32);

impl From<(u32, u32)> for Line {
    fn from((a, b): (u32, u32)) -> Self {
        Self(a, b)
    }
}

impl From<Line> for (u32, u32) {
    fn from(Line(a, b): Line) -> Self {
        (a, b)
    }
}

fn parse_line(i: &mut &str) -> PResult<Line> {
    separated_pair(dec_uint, space1, dec_uint)
        .parse_next(i)
        .map(Into::into)
}

fn parse_input(i: &mut &str) -> PResult<Vec<Line>> {
    repeat(1.., terminated(parse_line, opt(line_ending))).parse_next(i)
}

fn part1(input: &str) -> u32 {
    let lines: Vec<_> = parse_input.parse(input).unwrap();

    // dbg!(&lines);

    let (fst, snd): (Vec<_>, Vec<_>) = multiunzip(lines.into_iter().map(Into::into));

    izip!(fst.iter().sorted_unstable(), snd.iter().sorted_unstable())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(input: &str) -> u32 {
    let lines = parse_input.parse(input).unwrap();
    let (fst, snd): (Vec<_>, Vec<_>) = multiunzip(lines.into_iter().map(Into::into));
    let counts = snd.iter().counts();
    fst.iter()
        .map(|&i| i * *counts.get(&i).unwrap_or(&0) as u32)
        .sum()
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
