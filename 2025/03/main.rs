fn main() {
    let input = include_str!("./input.txt");

    println!(
        "Part 1 result is {}",
        largest_voltage_sum(parse_lines(input), 2)
    );

    println!(
        "Part 2 result is {}",
        largest_voltage_sum(parse_lines(input), 12)
    );
}

fn find_biggest_index_with_room(nums: &[u8], window: usize) -> usize {
    let n = nums.len();

    if window == 0 || n < window {
        return 0;
    }

    let mut biggest_idx = 0;

    for i in 0..=(n - window) {
        if nums[i] > nums[biggest_idx] {
            biggest_idx = i;
        }
    }

    biggest_idx
}

fn find_largest_voltage(batteries: &Vec<u8>, window: usize) -> u64 {
    let n = batteries.len();

    if window == 0 || window > n {
        return 0;
    }

    let mut sum = 0;
    let mut last_index = 0;

    for window_size in (1..=window).rev() {
        let slice = &batteries[last_index..];
        let biggest_index = last_index + find_biggest_index_with_room(slice, window_size);
        let biggest_num = batteries.get(biggest_index).unwrap();

        last_index = biggest_index + 1;

        sum = sum * 10 + *biggest_num as u64;
    }

    sum
}

fn largest_voltage_sum(rows: Vec<Vec<u8>>, window: usize) -> u64 {
    rows.iter()
        .map(|row| find_largest_voltage(row, window))
        .fold(0, |acc, v| acc + v)
}

fn parse_line(line: &str) -> Vec<u8> {
    line.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn parse_lines(lines: &str) -> Vec<Vec<u8>> {
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
            find_largest_voltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2)
        );

        assert_eq!(
            89,
            find_largest_voltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2)
        );

        assert_eq!(
            78,
            find_largest_voltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2)
        );

        assert_eq!(
            92,
            find_largest_voltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2)
        );
    }

    #[test]
    fn test_largest_voltage_sum() {
        assert_eq!(357, largest_voltage_sum(parse_lines(DEMO_INPUT), 2));
    }

    #[test]
    fn test_find_largest_voltage_12() {
        assert_eq!(
            987654321111,
            find_largest_voltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12)
        );

        assert_eq!(
            811111111119,
            find_largest_voltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12)
        );

        assert_eq!(
            434234234278,
            find_largest_voltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12)
        );

        assert_eq!(
            888911112111,
            find_largest_voltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12)
        );
    }

    #[test]
    fn test_largest_voltage_sum_12() {
        assert_eq!(
            3121910778619,
            largest_voltage_sum(parse_lines(DEMO_INPUT), 12)
        );
    }
}
