fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
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
