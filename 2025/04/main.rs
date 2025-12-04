fn main() {
    let input = include_str!("./input.txt");
    let g = Grid::from(input);

    println!("Part 1 result is {}", g.num_accessible_with(4),);
}

struct Grid {
    data: Vec<bool>,
    width: usize,
}

impl Grid {
    fn height(&self) -> usize {
        (self.data.len() + self.width - 1) / self.width
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn xy_from_index(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.data
            .get(self.index(x, y))
            .expect(format!("{x}x{y} isn't accessible").as_str())
            .clone()
    }

    fn num_adjacent_rolls(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;

        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                // ignore same
                if i == 0 && j == 0 {
                    continue;
                }

                let Some(x) = x.checked_add(i) else {
                    continue;
                };

                let Some(y) = y.checked_add(j) else {
                    continue;
                };

                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height() as i32 {
                    continue;
                }

                if !self.get(x as usize, y as usize) {
                    continue;
                }

                count += 1;
            }
        }

        count
    }

    fn num_accessible_with(&self, n: usize) -> usize {
        self.data.iter().enumerate().fold(0, |acc, (index, roll)| {
            if *roll {
                let (x, y) = self.xy_from_index(index);

                if self.num_adjacent_rolls(x, y) < n {
                    acc + 1
                } else {
                    acc
                }
            } else {
                acc
            }
        })
    }
}

impl<'a> From<&'a str> for Grid {
    fn from(value: &'a str) -> Self {
        let value = value.trim();
        let width = value.find('\n').unwrap_or(value.len());

        Grid {
            data: value
                .chars()
                .filter(|c| *c == '.' || *c == '@')
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect(),
            width,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Grid;

    const DEMO_INPUT: &'static str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_parse_input() {
        let g = Grid::from(DEMO_INPUT);
        assert_eq!(10, g.width);
        assert_eq!(false, g.get(0, 0));
        assert_eq!(true, g.get(2, 0));
    }

    #[test]
    fn test_num_adjacent_rolls() {
        let g = Grid::from(DEMO_INPUT);
        assert_eq!(2, g.num_adjacent_rolls(0, 0));
        assert_eq!(4, g.num_adjacent_rolls(1, 0));
        assert_eq!(1, g.num_adjacent_rolls(0, 9));
    }

    #[test]
    fn test_num_rolls_with_atleast() {
        let g = Grid::from(DEMO_INPUT);
        assert_eq!(13, g.num_accessible_with(4));
    }
}
