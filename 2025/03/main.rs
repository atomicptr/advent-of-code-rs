fn main() {
    let input = include_str!("./input.txt");

    println!(
        "Part 1 result is {}",
        largest_voltage_sum(parse_lines(input))
    );
}

fn find_largest_voltage(batteries: &Vec<usize>) -> usize {
    let mut max = 0;

    for i in 0..batteries.len() {
        for j in (i + 1)..batteries.len() {
            let comp = batteries.get(i).unwrap() * 10 + batteries.get(j).unwrap();

            if comp > max {
                max = comp;
            }
        }
    }

    max
}

fn largest_voltage_sum(rows: Vec<Vec<usize>>) -> usize {
    rows.iter()
        .map(|row| find_largest_voltage(row))
        .fold(0, |acc, v| acc + v)
}

fn parse_line(line: &str) -> Vec<usize> {
    line.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn parse_lines(lines: &str) -> Vec<Vec<usize>> {
    lines.trim().lines().map(|line| parse_line(line)).collect()
}

#[cfg(test)]
mod test {
    use crate::{find_largest_voltage, largest_voltage_sum, parse_lines};

    const DEMO_INPUT: &'static str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_find_largest_voltage() {
        assert_eq!(
            98,
            find_largest_voltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1])
        );

        assert_eq!(
            89,
            find_largest_voltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9])
        );

        assert_eq!(
            78,
            find_largest_voltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8])
        );

        assert_eq!(
            92,
            find_largest_voltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1])
        );
    }

    #[test]
    fn test_largest_voltage_sum() {
        assert_eq!(357, largest_voltage_sum(parse_lines(DEMO_INPUT)));
    }
}
