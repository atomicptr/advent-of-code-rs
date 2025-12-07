use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    let mut m = Machine::from(input);
    m.finish();

    println!("Part 1 result is {}", m.num_splits);

    let m = Machine::from(input);

    println!("Part 2 result is {}", m.finish_timelines());
}

#[derive(Debug, Clone)]
enum Cell {
    Start,
    Empty,
    Splitter,
    Beam,
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Cell::Start),
            '.' => Ok(Cell::Empty),
            '^' => Ok(Cell::Splitter),
            '|' => Ok(Cell::Beam),
            _ => Err(format!("Invalid Space character {value}")),
        }
    }
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Start => 'S',
            Cell::Empty => '.',
            Cell::Splitter => '^',
            Cell::Beam => '|',
        }
    }
}

#[derive(Clone)]
struct Machine {
    grid: Vec<Cell>,
    width: usize,
    beams: HashSet<(usize, usize)>,
    num_splits: usize,
}

#[derive(Debug)]
struct BeamResult {
    beams: HashSet<(usize, usize)>,
    split_beams: HashSet<(usize, usize)>,
    num_splits: usize,
}

impl<'a> From<&'a str> for Machine {
    fn from(value: &'a str) -> Self {
        let width = value.trim().find('\n').unwrap();

        let grid = value
            .chars()
            .filter_map(|c| Cell::try_from(c).ok())
            .collect::<Vec<Cell>>();

        let start_index = grid
            .iter()
            .enumerate()
            .find(|(_, cell)| matches!(cell, Cell::Start))
            .map(|(index, _)| index)
            .unwrap();

        let mut beams = HashSet::new();
        beams.insert(xy_from_index(width, start_index));

        Self {
            grid,
            width,
            beams,
            num_splits: 0,
        }
    }
}

impl Machine {
    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    fn height(&self) -> usize {
        (self.grid.len() + self.width - 1) / self.width
    }

    fn advance_beams(&self) -> BeamResult {
        let mut beams = HashSet::new();
        let mut split_beams = HashSet::new();
        let mut num_splits = 0;

        for &(beam_x, beam_y) in &self.beams {
            if beam_y + 1 >= self.height() {
                continue;
            }

            let new_y = beam_y + 1;
            let below = self.index(beam_x, new_y);

            match self.grid.get(below) {
                Some(Cell::Empty) => {
                    beams.insert((beam_x, new_y));
                }
                Some(Cell::Splitter) => {
                    let mut did_split = false;

                    // left
                    if let Some(left_x) = beam_x.checked_sub(1) {
                        if matches!(self.grid.get(self.index(left_x, new_y)), Some(Cell::Empty)) {
                            split_beams.insert((left_x, new_y));
                            did_split = true;
                        }
                    }

                    // right
                    if let Some(right_x) = beam_x.checked_add(1) {
                        if matches!(self.grid.get(self.index(right_x, new_y)), Some(Cell::Empty)) {
                            split_beams.insert((right_x, new_y));
                            did_split = true;
                        }
                    }

                    if did_split {
                        num_splits += 1;
                    }
                }
                _ => {}
            }
        }

        BeamResult {
            beams,
            split_beams,
            num_splits,
        }
    }

    fn step(&mut self) -> bool {
        let result = self.advance_beams();

        if !result.split_beams.is_empty() {
            self.num_splits += result.num_splits;
        }

        let mut new_beams = result.beams;
        new_beams.extend(result.split_beams);

        for &(x, y) in &new_beams {
            let idx = self.index(x, y);
            assert!(matches!(self.grid[idx], Cell::Empty));
            self.grid[idx] = Cell::Beam;
        }

        let has_new = !new_beams.is_empty();
        self.beams = new_beams;
        has_new
    }

    fn finish(&mut self) {
        while self.step() {}
    }

    fn finish_timelines(self) -> usize {
        let start = self.beams.iter().next().unwrap();
        let mut memo = HashMap::new();
        self.dfs(*start, &mut memo)
    }

    fn dfs(&self, (x, y): (usize, usize), memo: &mut HashMap<(usize, usize), usize>) -> usize {
        if y + 1 >= self.height() {
            return 1;
        }

        if let Some(&cached) = memo.get(&(x, y)) {
            return cached;
        }

        let below = self.index(x, y + 1);
        let result = match self.grid[below] {
            Cell::Empty => self.dfs((x, y + 1), memo),
            Cell::Splitter => {
                let mut sum = 0;

                if x > 0 {
                    let left = self.index(x - 1, y + 1);
                    if matches!(self.grid[left], Cell::Empty) {
                        sum += self.dfs((x - 1, y + 1), memo);
                    }
                }

                let right = self.index(x + 1, y + 1);
                if matches!(self.grid[right], Cell::Empty) {
                    sum += self.dfs((x + 1, y + 1), memo);
                }
                sum
            }
            _ => 0,
        };

        memo.insert((x, y), result);

        result
    }

    fn to_string(&self) -> String {
        let mut str = String::with_capacity(self.grid.len());

        for (index, cell) in self.grid.iter().enumerate() {
            if index > 0 && index % self.width == 0 {
                str.push('\n');
            }

            str.push(cell.to_char());
        }

        str
    }
}

fn xy_from_index(width: usize, index: usize) -> (usize, usize) {
    (index % width, index / width)
}

#[cfg(test)]
mod test {
    use crate::Machine;
    use pretty_assertions::assert_eq;

    const DEMO_INPUT: &'static str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_parse() {
        let m = Machine::from(DEMO_INPUT);
        assert_eq!(15, m.width);
    }

    #[test]
    fn test_steps() {
        let mut m = Machine::from(DEMO_INPUT);
        assert_eq!(DEMO_INPUT.trim(), m.to_string());

        assert!(m.step());
        assert_eq!(
            r#"
.......S.......
.......|.......
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#
            .trim(),
            m.to_string()
        );

        assert!(m.step());
        assert_eq!(
            r#"
.......S.......
.......|.......
......|^|......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#
            .trim(),
            m.to_string()
        );

        assert!(m.step());
        assert_eq!(
            r#"
.......S.......
.......|.......
......|^|......
......|.|......
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#
            .trim(),
            m.to_string()
        );

        assert!(m.step());
        assert_eq!(
            r#"
.......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#
            .trim(),
            m.to_string()
        );

        while m.step() {}

        assert_eq!(
            r#"
.......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
|.|.|.|.|.|||.|
"#
            .trim(),
            m.to_string()
        );
    }

    #[test]
    fn test_count_splits() {
        let mut m = Machine::from(DEMO_INPUT);
        m.finish();

        assert_eq!(21, m.num_splits);
    }

    #[test]
    fn test_timelines() {
        let m = Machine::from(DEMO_INPUT);
        assert_eq!(40, m.finish_timelines());
    }
}
