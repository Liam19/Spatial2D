use crate::*;

use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Direction::North => "North",
            Direction::East => "East",
            Direction::South => "South",
            Direction::West => "West",
            Direction::NorthEast => "North-East",
            Direction::SouthEast => "South-East",
            Direction::SouthWest => "South-West",
            Direction::NorthWest => "North-West",
        };
        write!(f, "{s}")
    }
}

impl Direction {
    #[inline]
    pub fn random() -> Self {
        let idx = Rng::new().gen_range(0..8);

        match idx {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            4 => Self::NorthEast,
            5 => Self::SouthEast,
            6 => Self::SouthWest,
            7 => Self::NorthWest,
            _ => panic!(),
        }
    }

    #[inline]
    pub fn random_no_diag() -> Self {
        let idx = Rng::new().gen_range(0..4);

        match idx {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!(),
        }
    }

    #[inline]
    pub fn random_diag_only() -> Self {
        let idx = Rng::new().gen_range(0..4);

        match idx {
            0 => Self::NorthEast,
            1 => Self::SouthEast,
            2 => Self::SouthWest,
            3 => Self::NorthWest,
            _ => panic!(),
        }
    }

    #[inline]
    pub const fn to_orientation(&self) -> Orientation {
        match self {
            Direction::North | Direction::South => Orientation::Vertical,
            Direction::East | Direction::West => Orientation::Horizontal,
            _ => panic!("Cannot get Orientation of diagonal Direction"),
        }
    }

    #[inline]
    pub const fn is_cardinal(&self) -> bool {
        match self {
            Direction::North | Direction::East | Direction::South | Direction::West => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn all() -> [Self; 8] {
        [
            Self::North,
            Self::South,
            Self::East,
            Self::West,
            Self::NorthEast,
            Self::SouthEast,
            Self::SouthWest,
            Self::NorthWest,
        ]
    }

    #[inline]
    pub const fn all_no_diag() -> [Self; 4] {
        [Self::North, Self::South, Self::East, Self::West]
    }

    #[inline]
    pub const fn all_diag_only() -> [Self; 4] {
        [
            Self::NorthEast,
            Self::SouthEast,
            Self::SouthWest,
            Self::NorthWest,
        ]
    }

    #[inline]
    pub const fn all_vectors() -> [IVec2; 8] {
        [
            Self::North.to_vector(),
            Self::South.to_vector(),
            Self::East.to_vector(),
            Self::West.to_vector(),
            Self::NorthEast.to_vector(),
            Self::SouthEast.to_vector(),
            Self::SouthWest.to_vector(),
            Self::NorthWest.to_vector(),
        ]
    }

    #[inline]
    pub const fn all_vectors_no_diag() -> [IVec2; 4] {
        [
            Self::North.to_vector(),
            Self::South.to_vector(),
            Self::East.to_vector(),
            Self::West.to_vector(),
        ]
    }

    #[inline]
    pub const fn all_vectors_diag_only() -> [IVec2; 4] {
        [
            Self::NorthEast.to_vector(),
            Self::SouthEast.to_vector(),
            Self::SouthWest.to_vector(),
            Self::NorthWest.to_vector(),
        ]
    }

    #[inline]
    pub const fn to_vector(self) -> IVec2 {
        match self {
            Self::North => IVec2::new(0, -1),
            Self::East => IVec2::new(1, 0),
            Self::South => IVec2::new(0, 1),
            Self::West => IVec2::new(-1, 0),
            Self::NorthEast => IVec2::new(1, -1),
            Self::SouthEast => IVec2::new(1, 1),
            Self::SouthWest => IVec2::new(-1, 1),
            Self::NorthWest => IVec2::new(-1, -1),
        }
    }

    #[inline]
    pub const fn from_vector(vector: IVec2) -> Self {
        match (vector.x, vector.y) {
            (0, -1) => Self::North,
            (1, 0) => Self::East,
            (0, 1) => Self::South,
            (-1, 0) => Self::West,
            (1, -1) => Self::NorthEast,
            (1, 1) => Self::SouthEast,
            (-1, 1) => Self::SouthWest,
            (-1, -1) => Self::NorthWest,
            _ => panic!(),
        }
    }

    #[must_use]
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast,
        }
    }

    #[must_use]
    #[inline]
    pub const fn turn_left_90(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::NorthEast => Self::NorthWest,
            Self::SouthEast => Self::NorthEast,
            Self::SouthWest => Self::SouthEast,
            Self::NorthWest => Self::SouthWest,
        }
    }

    #[must_use]
    #[inline]
    pub const fn turn_right_90(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::NorthEast => Self::SouthEast,
            Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest,
            Self::NorthWest => Self::NorthEast,
        }
    }

    #[must_use]
    #[inline]
    pub const fn turn_left_45(self) -> Self {
        match self {
            Self::North => Self::NorthWest,
            Self::East => Self::NorthEast,
            Self::South => Self::SouthEast,
            Self::West => Self::SouthWest,
            Self::NorthEast => Self::North,
            Self::SouthEast => Self::East,
            Self::SouthWest => Self::South,
            Self::NorthWest => Self::West,
        }
    }

    #[must_use]
    #[inline]
    pub const fn turn_right_45(self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::East => Self::SouthEast,
            Self::South => Self::SouthWest,
            Self::West => Self::NorthWest,
            Self::NorthEast => Self::East,
            Self::SouthEast => Self::South,
            Self::SouthWest => Self::West,
            Self::NorthWest => Self::North,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl From<Direction> for Orientation {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North | Direction::South => Self::Vertical,
            Direction::East | Direction::West => Self::Horizontal,
            _ => panic!(),
        }
    }
}

impl Orientation {
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Orientation::Vertical => Orientation::Horizontal,
            Orientation::Horizontal => Orientation::Vertical,
        }
    }

    pub fn random(rng: &mut Rng) -> Self {
        if rng.gen_bool(0.5) {
            Self::Vertical
        } else {
            Self::Horizontal
        }
    }
}
