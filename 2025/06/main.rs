fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1 result is {}", evaluate_sheet(input));
    println!("Part 2 result is {}", evaluate_sheet_cephalon(input));
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Op {
    Plus,
    Mul,
}

impl TryFrom<char> for Op {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Op::Plus),
            '*' => Ok(Op::Mul),
            s => Err(format!("Invalid operator '{s}'")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Num {
    Left(u64),
    Right(u64),
}

impl Num {
    fn value(&self) -> u64 {
        match self {
            Num::Left(v) => v.clone(),
            Num::Right(v) => v.clone(),
        }
    }
}

fn parse_sheet(rows: &str) -> (usize, Vec<Num>, Vec<Op>) {
    let rows = rows.trim().lines().collect::<Vec<&str>>();
    let (num_rows, ops_row) = rows.split_at(rows.len().saturating_sub(1));

    let ops_row = ops_row.first().unwrap();
    let ops: Vec<Op> = ops_row
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Op::try_from(c).unwrap())
        .collect();

    let width = ops.len();
    let mut numbers = Vec::with_capacity(width * rows.len());

    let col_width = num_rows
        .iter()
        .map(|s| {
            s.split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| s.len())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let col_width = (0..col_width[0].len())
        .map(|col| col_width.iter().map(|row| row[col]).max().unwrap())
        .collect::<Vec<usize>>();

    for line in num_rows {
        let mut taken = 0;

        for cw in &col_width {
            let num_str = &line[taken..(taken + cw).min(line.len())];
            taken += cw + 1;

            let is_right = *num_str.as_bytes().first().unwrap() == b' ';
            let val = num_str.trim().parse().unwrap();

            numbers.push(if is_right {
                Num::Right(val)
            } else {
                Num::Left(val)
            });
        }
    }

    (width, numbers, ops)
}

fn cephalon_col(col: Vec<Num>) -> Vec<u64> {
    let width = col
        .iter()
        .map(|n| n.value().to_string().len())
        .max()
        .unwrap_or(1);

    let lines = col
        .iter()
        .map(|n| match n {
            Num::Left(n) => format!("{:<width$}", n),
            Num::Right(n) => format!("{:>width$}", n),
        })
        .collect::<Vec<String>>();

    let mut numbers = Vec::new();

    for i in 0..width {
        let mut num = 0;

        for line in &lines {
            let Ok(n) = line[i..i + 1].parse::<u64>() else {
                continue;
            };

            num = num * 10 + n;
        }

        numbers.push(num);
    }

    numbers.reverse();
    numbers
}

fn evaluate_sheet(sheet: &str) -> u64 {
    let (width, num_rows, ops_row) = parse_sheet(sheet);
    let height = num_rows.len() / width;

    let make_index = |x: usize, y: usize| y * width + x;

    let mut res = Vec::with_capacity(width);

    for col in 0..width {
        let op = ops_row.get(col).unwrap();

        res.push(
            (0..height)
                .map(|row| num_rows.get(make_index(col, row)).unwrap())
                .fold(
                    match op {
                        Op::Plus => 0,
                        Op::Mul => 1,
                    },
                    |acc, val| match op {
                        Op::Plus => acc + val.value(),
                        Op::Mul => acc * val.value(),
                    },
                ),
        );
    }

    res.iter().fold(0, |acc, val| acc + val)
}

fn evaluate_sheet_cephalon(sheet: &str) -> u64 {
    let (width, num_rows, ops_row) = parse_sheet(sheet);
    let height = num_rows.len() / width;

    let make_index = |x: usize, y: usize| y * width + x;

    let mut res = Vec::with_capacity(width);

    for col in 0..width {
        let op = ops_row.get(col).unwrap();

        let col_numbers = (0..height)
            .map(|row| num_rows.get(make_index(col, row)).unwrap())
            .cloned()
            .collect::<Vec<Num>>();

        res.push(cephalon_col(col_numbers).iter().fold(
            match op {
                Op::Plus => 0,
                Op::Mul => 1,
            },
            |acc, val| match op {
                Op::Plus => acc + val,
                Op::Mul => acc * val,
            },
        ));
    }

    res.iter().fold(0, |acc, val| acc + val)
}

#[cfg(test)]
mod test {
    use crate::{cephalon_col, evaluate_sheet, evaluate_sheet_cephalon, parse_sheet, Num, Op};

    const DEMO_INPUT: &'static str = r#"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_parse_sheet() {
        let (width, numbers, ops) = parse_sheet(DEMO_INPUT);

        assert_eq!(4, width);
        assert_eq!(
            vec![
                Num::Left(123),
                Num::Left(328),
                Num::Right(51),
                Num::Left(64),
                Num::Right(45),
                Num::Left(64),
                Num::Left(387),
                Num::Left(23),
                Num::Right(6),
                Num::Left(98),
                Num::Left(215),
                Num::Left(314)
            ],
            numbers
        );
        assert_eq!(vec![Op::Mul, Op::Plus, Op::Mul, Op::Plus], ops);
    }

    #[test]
    fn test_evaluate_sheet() {
        assert_eq!(4277556, evaluate_sheet(DEMO_INPUT));
    }

    #[test]
    fn test_cephalon_col() {
        assert_eq!(
            vec![4, 431, 623],
            cephalon_col(vec![Num::Left(64), Num::Left(23), Num::Left(314)])
        );
        assert_eq!(
            vec![175, 581, 32],
            cephalon_col(vec![Num::Right(51), Num::Left(387), Num::Left(215)])
        );
        assert_eq!(
            vec![8, 248, 369],
            cephalon_col(vec![Num::Left(328), Num::Left(64), Num::Left(98)])
        );
        assert_eq!(
            vec![356, 24, 1],
            cephalon_col(vec![Num::Left(123), Num::Right(45), Num::Right(6)])
        );
    }

    #[test]
    fn test_evaluate_sheet_cephalon() {
        assert_eq!(3263827, evaluate_sheet_cephalon(DEMO_INPUT));
    }

    #[test]
    fn test_evaluate_sheet_cephalon2() {
        // part of my input cuz the normal test data wasnt enough to find a bug
        let input = r#"
744 65 616 826 486  4 878 2  99 1252
22  66 782 37  128  5 44  15 45 8823
6   32 331 87  672 49 18  26 37 5138
5    5 681 87  51  13 97  66 1  934
*   +  +   +   *   *  +   *  +  +"#;

        assert_eq!(
            (4 * 42 * 7265)
                + (5625 + 663)
                + (6211 + 1838 + 6736)
                + (6 + 2777 + 8388)
                + (682 * 8271 * 4165)
                + (4593 * 41)
                + (8 + 7487 + 8419)
                + (566 * 2126)
                + (957 + 9431)
                + (238 + 5234 + 2813 + 1859),
            evaluate_sheet_cephalon(input)
        );
    }
}
