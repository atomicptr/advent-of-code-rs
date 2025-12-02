fn main() {
    let input = include_str!("./input.txt");
    println!(
        "Part 1 result is {}",
        sum_invalid_ids(parse_input(input), find_invalid_ids_pair)
    );
    println!(
        "Part 2 result is {}",
        sum_invalid_ids(parse_input(input), find_invalid_ids_multiple)
    );
}

fn num_digits(n: &u64) -> usize {
    (n.checked_ilog10().unwrap_or(0) + 1) as usize
}

fn split_num(n: &u64, parts: usize) -> Option<Vec<u64>> {
    // no parts? Drop it
    if parts == 0 {
        return None;
    }

    let digits = num_digits(n);

    // if isnt divisible cleanly, drop it
    if digits % parts != 0 {
        return None;
    }

    let chunk = digits / parts;
    let div = 10u64.pow(chunk as u32);
    let mut res = Vec::with_capacity(parts);
    let mut x = n.clone();

    for _ in 1..parts {
        res.push(x % div);
        x /= div;
    }

    res.push(x);

    Some(res)
}

fn power_split(n: &u64) -> Vec<Vec<u64>> {
    let digits = num_digits(n);
    let mut res = Vec::new();

    for parts in 1..=digits {
        if digits % parts != 0 {
            continue;
        }

        if let Some(split) = split_num(&n, parts) {
            res.push(split);
        }
    }

    res
}

fn find_invalid_ids_pair(from: u64, to: u64) -> Vec<u64> {
    (from..=to)
        .filter(|num| match split_num(num, 2).as_deref() {
            Some([left, right]) => left == right,
            _ => false,
        })
        .collect()
}

fn find_invalid_ids_multiple(from: u64, to: u64) -> Vec<u64> {
    (from..=to)
        .filter(|num| {
            power_split(num)
                .iter()
                .filter(|v| v.len() >= 2)
                .any(|v| v.windows(2).all(|w| w[0] == w[1]))
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|s| s.trim().split_once('-'))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn sum_invalid_ids<F>(ranges: Vec<(u64, u64)>, f: F) -> u64
where
    F: Fn(u64, u64) -> Vec<u64>,
{
    ranges
        .iter()
        .map(|(from, to)| f(from.clone(), to.clone()))
        .flatten()
        .fold(0, |acc, id| acc + id)
}

#[cfg(test)]
mod test {
    use crate::{find_invalid_ids_multiple, find_invalid_ids_pair, parse_input, sum_invalid_ids};

    const DEMO_INPUT: &'static str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"#;

    #[test]
    fn test_find_invalid_ids() {
        let f = find_invalid_ids_pair;

        assert_eq!(vec![11, 22], f(11, 22));
        assert_eq!(vec![99], f(95, 115));
        assert_eq!(vec![1010], f(998, 1012));
        assert_eq!(vec![222222], f(222220, 222224));
        assert_eq!(vec![1188511885], f(1188511880, 1188511890));
        assert!(f(1698522, 1698528).is_empty());
        assert_eq!(vec![446446], f(446443, 446449));
        assert_eq!(vec![38593859], f(38593856, 38593862));
        assert_eq!(vec![38593859], f(38593856, 38593862));
        assert!(f(565653, 565659).is_empty());
        assert!(f(824824821, 824824827).is_empty());
        assert!(f(2121212118, 2121212124).is_empty());
    }

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(
            1227775554,
            sum_invalid_ids(parse_input(DEMO_INPUT), find_invalid_ids_pair)
        );
    }

    #[test]
    fn test_find_invalid_ids_multiple() {
        let f = find_invalid_ids_multiple;

        assert_eq!(vec![11, 22], f(11, 22));
        assert_eq!(vec![99, 111], f(95, 115));
        assert_eq!(vec![999, 1010], f(998, 1012));
        assert_eq!(vec![1188511885], f(1188511880, 1188511890));
        assert_eq!(vec![222222], f(222220, 222224));
        assert!(f(1698522, 1698528).is_empty());
        assert_eq!(vec![446446], f(446443, 446449));
        assert_eq!(vec![38593859], f(38593856, 38593862));
        assert_eq!(vec![38593859], f(38593856, 38593862));
        assert_eq!(vec![565656], f(565653, 565659));
        assert_eq!(vec![824824824], f(824824821, 824824827));
        assert_eq!(vec![2121212121], f(2121212118, 2121212124));
    }
}
