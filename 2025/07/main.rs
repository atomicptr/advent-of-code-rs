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
    beams: Vec<(usize, usize)>,
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

        Self {
            grid,
            width,
            beams: vec![xy_from_index(width, start_index)],
            num_splits: 0,
        }
    }
}

impl Machine {
    fn xy_from_index(&self, index: usize) -> (usize, usize) {
        xy_from_index(self.width, index)
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn height(&self) -> usize {
        (self.grid.len() + self.width - 1) / self.width
    }

    fn is_empty(&self, index: usize) -> bool {
        self.grid
            .get(index)
            .map(|cell| matches!(cell, Cell::Empty))
            .unwrap_or(false)
    }

    fn set(&mut self, index: usize, cell: Cell) {
        assert!(matches!(self.grid.get(index).unwrap(), Cell::Empty));

        self.grid[index] = cell;
    }

    fn clone_with_new_beam(&self, (x, y): (usize, usize)) -> Self {
        let index = self.index(x, y);
        let mut new = self.clone();

        assert!(matches!(new.grid[index], Cell::Empty));
        new.grid[index] = Cell::Beam;
        new.beams.clear();
        new.beams.push((x, y));
        new.num_splits = 0;

        new
    }

    fn split_timeline(mut self) -> Vec<Self> {
        let mut timelines = vec![];
        let mut new_beams = vec![];

        for (beam_x, beam_y) in &self.beams {
            if beam_y + 1 >= self.height() {
                continue;
            }

            let below_beam = self.index(beam_x.clone(), beam_y + 1);

            let beam_y = beam_y + 1;

            match self.grid.get(below_beam) {
                Some(Cell::Empty) => new_beams.push((*beam_x, beam_y)),
                Some(Cell::Splitter) => {
                    // left beam
                    if let Some(new_beam_x) = beam_x.checked_sub(1) {
                        if let Some(cell) = self.grid.get(self.index(new_beam_x, beam_y))
                            && matches!(cell, Cell::Empty)
                        {
                            timelines.push(self.clone_with_new_beam((new_beam_x, beam_y)));
                        }
                    }

                    // right beam
                    if let Some(new_beam_x) = beam_x.checked_add(1) {
                        if let Some(cell) = self.grid.get(self.index(new_beam_x, beam_y))
                            && matches!(cell, Cell::Empty)
                        {
                            timelines.push(self.clone_with_new_beam((new_beam_x, beam_y)));
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        // make sure we dont have duplicates
        let new_beams: Vec<_> = new_beams
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for (beam_x, beam_y) in &new_beams {
            let index = self.index(*beam_x, *beam_y);
            if let Some(cell) = self.grid.get(index) {
                assert!(
                    matches!(cell, Cell::Empty),
                    "Expected empty at {beam_x}x{beam_y} but found {cell:?}"
                );

                self.grid[index] = Cell::Beam;
            }
        }

        let timeline_still_alive = new_beams.len() > 0;

        self.beams.extend(new_beams);

        // continue our timeline if something happened
        if timeline_still_alive {
            timelines.push(self);
        }

        timelines
    }

    fn step(&mut self) -> bool {
        let mut new_beams = vec![];

        for (beam_x, beam_y) in &self.beams {
            if beam_y + 1 >= self.height() {
                continue;
            }

            let below_beam = self.index(beam_x.clone(), beam_y + 1);

            let beam_y = beam_y + 1;

            match self.grid.get(below_beam) {
                Some(Cell::Empty) => new_beams.push((*beam_x, beam_y)),
                Some(Cell::Splitter) => {
                    let mut did_split = false;

                    // left beam
                    if let Some(new_beam_x) = beam_x.checked_sub(1) {
                        if let Some(cell) = self.grid.get(self.index(new_beam_x, beam_y))
                            && matches!(cell, Cell::Empty)
                        {
                            new_beams.push((new_beam_x, beam_y));
                            did_split = true;
                        }
                    }

                    // right beam
                    if let Some(new_beam_x) = beam_x.checked_add(1) {
                        if let Some(cell) = self.grid.get(self.index(new_beam_x, beam_y))
                            && matches!(cell, Cell::Empty)
                        {
                            new_beams.push((new_beam_x, beam_y));
                            did_split = true;
                        }
                    }

                    if did_split {
                        self.num_splits += 1;
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        let has_new_beams = new_beams.len() > 0;

        // make sure we dont have duplicates
        let new_beams: Vec<_> = new_beams
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for (beam_x, beam_y) in &new_beams {
            let index = self.index(*beam_x, *beam_y);
            if let Some(cell) = self.grid.get(index) {
                assert!(
                    matches!(cell, Cell::Empty),
                    "Expected empty at {beam_x}x{beam_y} but found {cell:?}"
                );

                self.grid[index] = Cell::Beam;
            }
        }

        self.beams.extend(new_beams);

        has_new_beams
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
