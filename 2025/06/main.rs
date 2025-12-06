fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1 result is {}", evaluate_sheet(input));
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Op {
    Plus,
    Mul,
}

impl<'a> TryFrom<&'a str> for Op {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Op::Plus),
            "*" => Ok(Op::Mul),
            s => Err(format!("Invalid operator '{s}'")),
        }
    }
}

fn split_line_numbers(line: &str) -> Vec<u64> {
    line.split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn split_line_ops(line: &str) -> Vec<Op> {
    line.split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| Op::try_from(s).unwrap())
        .collect()
}

fn evaluate_sheet(sheet: &str) -> u64 {
    let lines = sheet.trim().lines().collect::<Vec<&str>>();
    let count_numbers = lines.len() - 1;

    let width = split_line_numbers(lines.first().unwrap()).len();

    let mut sheet = Vec::with_capacity(width * count_numbers);
    let mut ops = Vec::with_capacity(width);

    let make_index = |x: usize, y: usize| y * width + x;

    for (index, line) in lines.iter().enumerate() {
        if index < count_numbers {
            let numbers = split_line_numbers(line);

            for num in numbers {
                sheet.push(num);
            }
        } else {
            for op in split_line_ops(line) {
                ops.push(op);
            }
            break;
        }
    }

    let mut res = Vec::with_capacity(width);

    for col in 0..width {
        let op = ops.get(col).unwrap();

        res.push(
            (0..count_numbers)
                .map(|row| sheet.get(make_index(col, row)).unwrap())
                .fold(
                    match op {
                        Op::Plus => 0,
                        Op::Mul => 1,
                    },
                    |acc, val| match op {
                        Op::Plus => acc + val,
                        Op::Mul => acc * val,
                    },
                ),
        );
    }

    res.iter().fold(0, |acc, val| acc + val)
}

#[cfg(test)]
mod test {
    use crate::{evaluate_sheet, split_line_numbers, split_line_ops, Op};

    const DEMO_INPUT: &'static str = r#"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_split_line_numbers() {
        assert_eq!(vec![123, 328, 51, 64], split_line_numbers("123 328  51 64"));
        assert_eq!(vec![45, 64, 387, 23], split_line_numbers(" 45 64  387 23"));
        assert_eq!(vec![6, 98, 215, 314], split_line_numbers("  6 98  215 314"));
    }

    #[test]
    fn test_split_line_ops() {
        assert_eq!(
            vec![Op::Mul, Op::Plus, Op::Mul, Op::Plus],
            split_line_ops("*   +   *   +  ")
        );
    }

    #[test]
    fn test_evaluate_sheet() {
        assert_eq!(4277556, evaluate_sheet(DEMO_INPUT));
    }
}
