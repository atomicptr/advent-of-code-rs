fn main() {
    let input = include_str!("./input.txt");
    let rots = parse_rot_seq(input);

    println!(
        "Part 1 result is: {}",
        count_state_reached(Safe::new(50), &rots, 0)
    );

    println!(
        "Part 2 result is: {}",
        Safe::new(50).apply_rotations(&rots).clicks
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rot {
    Left(i16),
    Right(i16),
}

impl From<String> for Rot {
    fn from(value: String) -> Self {
        let dir = value
            .chars()
            .take(1)
            .next()
            .expect("could not read direction");
        let value = value
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("could not read value");

        match dir {
            'L' => Rot::Left(value),
            'R' => Rot::Right(value),
            _ => panic!("Invalid direction {dir}"),
        }
    }
}

pub fn parse_rot_seq(input: &str) -> Vec<Rot> {
    input.lines().map(|line| line.to_string().into()).collect()
}

pub struct Safe {
    pub current: u8,
    pub clicks: u32,
}

impl Safe {
    pub fn new(current: u8) -> Self {
        Safe { current, clicks: 0 }
    }

    pub fn with_clicks(self, clicks: u32) -> Safe {
        Safe {
            current: self.current,
            clicks,
        }
    }

    pub fn apply_rotation(self, rot: Rot) -> Self {
        match rot {
            Rot::Left(val) => self.turn(-i16::from(val)),
            Rot::Right(val) => self.turn(i16::from(val)),
        }
    }

    pub fn apply_rotations(self, rots: &Vec<Rot>) -> Self {
        rots.iter()
            .cloned()
            .fold(self, |safe, rot| safe.apply_rotation(rot))
    }

    fn turn(self, val: i16) -> Self {
        let mut current = self.current as i16;
        let mut clicks = self.clicks;

        let dir = if val >= 0 { 1 } else { -1 };

        for _ in 0..val.abs() {
            current = (current + dir).rem_euclid(100);

            if current == 0 {
                clicks += 1;
            }
        }

        Safe::new(u8::try_from(current).expect("invalid state")).with_clicks(clicks)
    }
}

fn count_state_reached(safe: Safe, rots: &Vec<Rot>, value: u8) -> u32 {
    rots.iter()
        .cloned()
        .fold((safe, 0), |(safe, curr), rot| {
            let safe = safe.apply_rotation(rot);
            let v = safe.current;

            (safe, if v == value { curr + 1 } else { curr })
        })
        .1
}

#[cfg(test)]
mod test {
    use crate::{count_state_reached, parse_rot_seq, Rot, Safe};

    const DEMO_INPUT: &'static str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_parsing() {
        for i in 0..=99 {
            assert_eq!(Rot::Left(i), Rot::from(format!("L{i}")));
            assert_eq!(Rot::Right(i), Rot::from(format!("R{i}")));
        }
    }

    #[test]
    fn test_parsing_lines() {
        assert_eq!(
            vec![
                Rot::Left(68),
                Rot::Left(30),
                Rot::Right(48),
                Rot::Left(5),
                Rot::Right(60),
                Rot::Left(55),
                Rot::Left(1),
                Rot::Left(99),
                Rot::Right(14),
                Rot::Left(82)
            ],
            parse_rot_seq(DEMO_INPUT),
        );
    }

    #[test]
    fn test_safe_apply_rotations() {
        assert_eq!(19, Safe::new(11).apply_rotation(Rot::Right(8)).current);
        assert_eq!(5, Safe::new(11).apply_rotation(Rot::Left(6)).current);
        assert_eq!(
            0,
            Safe::new(11)
                .apply_rotation(Rot::Right(8))
                .apply_rotation(Rot::Left(19))
                .current
        );
        assert_eq!(99, Safe::new(0).apply_rotation(Rot::Left(1)).current);
        assert_eq!(0, Safe::new(99).apply_rotation(Rot::Right(1)).current);
        assert_eq!(50, Safe::new(50).apply_rotation(Rot::Left(0)).current);
        assert_eq!(50, Safe::new(50).apply_rotation(Rot::Right(0)).current);
    }

    #[test]
    fn test_part_1_example() {
        let rots = parse_rot_seq(DEMO_INPUT);
        assert_eq!(3, count_state_reached(Safe::new(50), &rots, 0));
    }

    #[test]
    fn test_part_2_example() {
        let rots = parse_rot_seq(DEMO_INPUT);
        assert_eq!(6, Safe::new(50).apply_rotations(&rots).clicks);
    }

    #[test]
    fn test_big_rotation_clicks() {
        let res = Safe::new(50).apply_rotation(Rot::Right(1000));
        assert_eq!(50, res.current);
        assert_eq!(10, res.clicks);
    }
}
