fn main() {
    let input = include_str!("./input.txt");

    let inv = Inventory::from(input);

    println!("Part 1 result is {}", inv.count_still_fresh_ingredients());
    println!("Part 2 result is {}", inv.count_fresh());
}

struct Inventory {
    fresh: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Inventory {
    fn is_fresh(&self, num: &u64) -> bool {
        for (from, to) in &self.fresh {
            if (from..=to).contains(&&num) {
                return true;
            }
        }

        false
    }

    fn count_still_fresh_ingredients(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|num| self.is_fresh(*num))
            .count()
    }

    fn count_fresh(self) -> u64 {
        let fresh = merge_ranges(self.fresh);
        fresh
            .iter()
            .map(|(from, to)| to - from + 1)
            .fold(0, |acc, n| acc + n)
    }
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|r| r.0);
    ranges
        .into_iter()
        .fold(vec![], |mut acc: Vec<(u64, u64)>, curr| {
            match acc.last_mut() {
                Some(last) if last.1 >= curr.0 - 1 => last.1 = last.1.max(curr.1),
                _ => acc.push(curr),
            }
            acc
        })
}

fn split_at_empty_line(input: &str) -> (&str, &str) {
    let bytes = input.as_bytes();

    for i in 1..bytes.len() {
        if bytes[i] == b'\n' && bytes[i - 1] == b'\n' {
            return (&input[..i], &input[i..]);
        }
    }

    (input, "")
}

impl<'a> From<&'a str> for Inventory {
    fn from(value: &'a str) -> Self {
        let (fresh, available) = split_at_empty_line(value.trim());

        let mut inv = Inventory {
            fresh: Vec::with_capacity(fresh.len()),
            ingredients: Vec::with_capacity(available.len()),
        };

        for line in fresh.trim().lines() {
            let (from, to) = line.split_once("-").unwrap();
            inv.fresh.push((from.parse().unwrap(), to.parse().unwrap()));
        }

        for line in available.trim().lines() {
            inv.ingredients.push(line.parse().unwrap());
        }

        inv
    }
}

#[cfg(test)]
mod test {
    use crate::Inventory;

    const DEMO_INPUT: &'static str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_parsing() {
        let inv = Inventory::from(DEMO_INPUT);

        assert_eq!(vec![(3, 5), (10, 14), (16, 20), (12, 18)], inv.fresh);
        assert_eq!(vec![1, 5, 8, 11, 17, 32], inv.ingredients);
    }

    #[test]
    fn test_count_still_fresh_ingredients() {
        let inv = Inventory::from(DEMO_INPUT);
        assert_eq!(3, inv.count_still_fresh_ingredients());
    }

    #[test]
    fn test_count_fresh() {
        let inv = Inventory::from(DEMO_INPUT);
        assert_eq!(14, inv.count_fresh());
    }
}
