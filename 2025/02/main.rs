fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1 result is {}", sum_invalid_ids(parse_input(input)));
}

fn split_num(n: &u64) -> Option<(u64, u64)> {
    let digits = n.checked_ilog10().unwrap_or(0) + 1;

    if digits % 2 == 0 {
        let mid = (digits + 1) / 2;
        let div = 10u64.pow(mid);
        Some((n / div, n % div))
    } else {
        None
    }
}

fn rule_duplicated_sequence(num: &u64) -> bool {
    match split_num(num) {
        Some((left, right)) => left == right,
        None => false,
    }
}

fn find_invalid_ids(from: u64, to: u64) -> Vec<u64> {
    (from..=to).filter(rule_duplicated_sequence).collect()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|s| s.trim().split_once('-'))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn sum_invalid_ids(ranges: Vec<(u64, u64)>) -> u64 {
    ranges
        .iter()
        .map(|(from, to)| find_invalid_ids(from.clone(), to.clone()))
        .flatten()
        .fold(0, |acc, id| acc + id)
}

#[cfg(test)]
mod test {
    use crate::{find_invalid_ids, parse_input, sum_invalid_ids};

    const DEMO_INPUT: &'static str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"#;

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(vec![11, 22], find_invalid_ids(11, 22));
        assert_eq!(vec![99], find_invalid_ids(95, 115));
        assert_eq!(vec![1010], find_invalid_ids(998, 1012));
        assert_eq!(vec![1188511885], find_invalid_ids(1188511880, 1188511890));
        assert!(find_invalid_ids(1698522, 1698528).is_empty());
        assert_eq!(vec![446446], find_invalid_ids(446443, 446449));
        assert_eq!(vec![38593859], find_invalid_ids(38593856, 38593862));
        assert_eq!(vec![38593859], find_invalid_ids(38593856, 38593862));
        assert!(find_invalid_ids(565653, 565659).is_empty());
        assert!(find_invalid_ids(824824821, 824824827).is_empty());
        assert!(find_invalid_ids(2121212118, 2121212124).is_empty());
    }

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(1227775554, sum_invalid_ids(parse_input(DEMO_INPUT)));
    }
}
