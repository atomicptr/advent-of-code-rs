use std::collections::HashSet;

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

    fn clone_with_new_beam(&self, (x, y): (usize, usize)) -> Self {
        let index = self.index(x, y);
        assert!(matches!(self.grid[index], Cell::Empty));

        let mut beams = HashSet::new();
        beams.insert((x, y));

        Self {
            grid: self.grid.clone(),
            width: self.width,
            beams,
            num_splits: 0,
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

    fn split_timeline(mut self) -> Vec<Self> {
        let result = self.advance_beams();
        let mut timelines = Vec::new();

        // for each split produce a new timeline
        for &(x, y) in &result.split_beams {
            timelines.push(self.clone_with_new_beam((x, y)));
        }

        for &(x, y) in &result.beams {
            let idx = self.index(x, y);
            assert!(matches!(self.grid[idx], Cell::Empty));
            self.grid[idx] = Cell::Beam;
        }

        if !result.beams.is_empty() {
            self.beams = result.beams;
            timelines.push(self);
        }

        timelines
    }

    fn finish(&mut self) {
        while self.step() {}
    }

    fn finish_timelines(self) -> usize {
        let mut active = vec![self];
        let mut dead = 0;

        while !active.is_empty() {
            let mut next = Vec::new();

            for timeline in active {
                let branches = timeline.split_timeline();

                if branches.is_empty() {
                    dead += 1;
                }

                for branch in branches {
                    next.push(branch);
                }
            }

            active = next;
        }

        dead
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
