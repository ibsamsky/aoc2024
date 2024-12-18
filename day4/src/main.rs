use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Cell {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn try_move(&self, d: (isize, isize)) -> Option<Point> {
        Some(Point {
            x: self.x.checked_add_signed(d.0)?,
            y: self.y.checked_add_signed(d.1)?,
        })
    }
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines = input.lines();
        let cells: Vec<_> = lines
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    'X' => Cell::X,
                    'M' => Cell::M,
                    'A' => Cell::A,
                    'S' => Cell::S,
                    _ => panic!("Invalid character in input"),
                })
            })
            .collect();

        // grid is a square(?)
        let width = input.lines().next().unwrap().len();
        let height = width;

        Self {
            width,
            height,
            cells,
        }
    }

    // offset into the cells vector
    fn cell_index(&self, p: Point) -> Option<usize> {
        let Point { x, y } = p;
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }

    fn cell(&self, p: Point) -> Option<&Cell> {
        Some(&self.cells[self.cell_index(p)?])
    }
}

#[rustfmt::skip]
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),/*(0, 0),*/( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1),
];

#[rustfmt::skip]
const DIAGONALS: [(isize, isize); 4] = [
    (-1, -1), ( 1, -1),
    (-1,  1), ( 1,  1),
];

fn x_forms_xmas(grid: &Grid, x_point: Point) -> Option<u8> {
    if !matches!(grid.cell(x_point), Some(&Cell::X)) {
        return None;
    }

    let m_points: Vec<_> = DIRECTIONS
        .into_iter()
        .filter_map(|d| Some((d, x_point.try_move(d)?)))
        .filter(|(_, p)| matches!(grid.cell(*p), Some(&Cell::M)))
        .collect();

    let mut xmas_total = 0;
    for (d, p) in m_points {
        // cannot use `?` here because it would return from the function
        if let Some(a_point) = p.try_move(d) {
            if !matches!(grid.cell(a_point), Some(&Cell::A)) {
                continue;
            }
            if let Some(s_point) = a_point.try_move(d) {
                if !matches!(grid.cell(s_point), Some(&Cell::S)) {
                    continue;
                }
                xmas_total += 1;
            }
        }
    }
    Some(xmas_total)
}

fn a_forms_x_mas(grid: &Grid, a_point: Point) -> Option<u8> {
    // deny points on the edge
    if matches!(a_point, Point { x, y } if x == 0 || y == 0 || x == grid.width - 1 || y == grid.height - 1)
    {
        return None;
    }
    if !matches!(grid.cell(a_point), Some(&Cell::A)) {
        return None;
    }

    let (m1, m2) = DIAGONALS
        .into_iter()
        .tuple_combinations()
        .filter_map(|(d1, d2)| {
            // fine to use `?` here due to closure, plus any valid A point will not be on the edge
            let p1 = a_point.try_move(d1)?;
            let p2 = a_point.try_move(d2)?;
            if matches!(grid.cell(p1), Some(&Cell::M)) && matches!(grid.cell(p2), Some(&Cell::M)) {
                Some((d1, d2))
            } else {
                None
            }
        })
        .next()?; // there can only be one pair (the other two diagonals are taken by S)

    // invert over the A point
    let (d1, d2) = ((-m1.0, -m1.1), (-m2.0, -m2.1));
    let s1 = a_point.try_move(d1)?;
    let s2 = a_point.try_move(d2)?;

    if matches!(grid.cell(s1), Some(&Cell::S)) && matches!(grid.cell(s2), Some(&Cell::S)) {
        Some(1)
    } else {
        Some(0) // or None, doesn't matter
    }
}

fn part1(input: &str) -> u16 {
    let grid = Grid::parse(input);
    (0..grid.width)
        .cartesian_product(0..grid.height)
        .map(|(x, y)| Point { x, y })
        .filter_map(|p| x_forms_xmas(&grid, p).map(|x| x as u16))
        .sum::<u16>()
}

fn part2(input: &str) -> u16 {
    let grid = Grid::parse(input);
    (0..grid.width)
        .cartesian_product(0..grid.height)
        .map(|(x, y)| Point { x, y })
        .filter_map(|p| a_forms_x_mas(&grid, p).map(|x| x as u16))
        .sum::<u16>()
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
