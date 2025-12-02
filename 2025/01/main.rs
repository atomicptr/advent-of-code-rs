fn main() {
    let input = include_str!("./input.txt");
    let rots = parse_rot_seq(input);

    println!(
        "Part 1 result is: {}",
        count_state_reached(Safe(50), rots, 0)
    );
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

pub struct Safe(pub u8);

impl Safe {
    pub fn apply_rotation(self, rot: Rot) -> Self {
        match rot {
            Rot::Left(val) => self.turn(-i16::from(val)),
            Rot::Right(val) => self.turn(i16::from(val)),
        }
    }

    fn turn(self, val: i16) -> Self {
        Safe(
            (self.0 as i16 + val as i16)
                .rem_euclid(100)
                .try_into()
                .expect("invalid state"),
        )
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

fn count_state_reached(safe: Safe, rots: Vec<Rot>, value: u8) -> u32 {
    rots.iter()
        .cloned()
        .fold((safe, 0), |(safe, curr), rot| {
            let safe = safe.apply_rotation(rot);
            let v = safe.0;

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
        assert_eq!(19, Safe(11).apply_rotation(Rot::Right(8)).value());
        assert_eq!(5, Safe(11).apply_rotation(Rot::Left(6)).value());
        assert_eq!(
            0,
            Safe(11)
                .apply_rotation(Rot::Right(8))
                .apply_rotation(Rot::Left(19))
                .value()
        );
        assert_eq!(99, Safe(0).apply_rotation(Rot::Left(1)).value());
        assert_eq!(0, Safe(99).apply_rotation(Rot::Right(1)).value());
        assert_eq!(50, Safe(50).apply_rotation(Rot::Left(0)).value());
        assert_eq!(50, Safe(50).apply_rotation(Rot::Right(0)).value());
    }

    #[test]
    fn test_part_1_example() {
        let rots = parse_rot_seq(DEMO_INPUT);
        assert_eq!(3, count_state_reached(Safe(50), rots, 0))
    }
}
