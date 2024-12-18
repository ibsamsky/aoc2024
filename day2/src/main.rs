use itertools::Itertools;
use winnow::ascii::{dec_uint, line_ending, space1};
use winnow::combinator::{opt, repeat, terminated};
use winnow::seq;
use winnow::{combinator::separated, prelude::*};

#[derive(Debug)]
struct Report {
    levels: Vec<u8>,
}

fn parse_report(i: &mut &str) -> PResult<Report> {
    seq!(Report {
        levels: separated(1.., dec_uint::<_, u8, _>, space1)
    })
    .parse_next(i)
}

fn parse_input(i: &mut &str) -> PResult<Vec<Report>> {
    repeat(1.., terminated(parse_report, opt(line_ending))).parse_next(i)
}

fn diffs(i: &[u8]) -> Vec<i8> {
    i.iter()
        .tuple_windows()
        .map(|(a, b)| *b as i8 - *a as i8)
        .collect()
}

fn monotonic(i: &[u8]) -> bool {
    let diffs = diffs(i);
    let sign = diffs[0].signum();
    diffs
        .iter()
        .all(|&x| x.signum() == sign && (1..=3).contains(&x.abs()))
}

fn monotonic_with_tolerance(i: &[u8]) -> bool {
    i.iter()
        .copied()
        .combinations(i.len() - 1)
        .any(|l| monotonic(&l))
}

fn part1(input: &str) -> u32 {
    let reports = parse_input.parse(input).unwrap();
    reports.iter().filter(|r| monotonic(&r.levels)).count() as u32
}

fn part2(input: &str) -> u32 {
    let reports = parse_input.parse(input).unwrap();
    reports
        .iter()
        // just check with one removed anyway! there's probably less branching too
        .filter(|r| monotonic_with_tolerance(&r.levels))
        .count() as u32
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
