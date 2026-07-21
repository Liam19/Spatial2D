use crate::*;

use Dir::*;
use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Dir {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

impl From<Dir> for u8 {
    #[inline(always)]
    fn from(dir: Dir) -> u8 {
        dir as u8 // Free at runtime
    }
}

impl TryFrom<u8> for Dir {
    type Error = ();

    #[inline(always)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 8 {
            // SAFE: We just checked bounds, and Dir is repr(u8)
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            N => "N",
            NE => "NE",
            E => "E",
            SE => "SE",
            S => "S",
            SW => "SW",
            W => "W",
            NW => "NW",
        };
        write!(f, "{s}")
    }
}

impl Dir {
    pub const ALL: [Self; 8] = [N, NE, E, SE, S, SW, W, NW];

    pub const ALL_CARDINAL: [Self; 4] = [N, E, S, W];

    pub const ALL_DIAG: [Self; 4] = [NE, SE, SW, NW];

    pub const ALL_VECTORS: [IVec2; 8] = [
        IVec2::new(0, -1),  // N  = 0
        IVec2::new(1, -1),  // NE = 1
        IVec2::new(1, 0),   // E  = 2
        IVec2::new(1, 1),   // SE = 3
        IVec2::new(0, 1),   // S  = 4
        IVec2::new(-1, 1),  // SW = 5
        IVec2::new(-1, 0),  // W  = 6
        IVec2::new(-1, -1), // NW = 7
    ];

    pub const ALL_VECTORS_CARDINAL: [IVec2; 4] = [
        IVec2::new(0, -1), // N
        IVec2::new(1, 0),  // E
        IVec2::new(0, 1),  // S
        IVec2::new(-1, 0), // W
    ];

    pub const ALL_VECTORS_DIAG: [IVec2; 4] = [
        IVec2::new(1, -1),  // NE
        IVec2::new(1, 1),   // SE
        IVec2::new(-1, 1),  // SW
        IVec2::new(-1, -1), // NW
    ];

    pub const fn to_vector(self) -> IVec2 {
        Self::ALL_VECTORS[self as usize]
    }

    /// 3x3 lookup table indexed by [y + 1][x + 1]
    #[rustfmt::skip]
    const FROM_VEC_LUT: [Option<Dir>; 9] = [
        // y = -1
        Some(Dir::NW), Some(Dir::N), Some(Dir::NE), // x = -1, 0, 1
        // y = 0
        Some(Dir::W),  None,         Some(Dir::E),  // x = -1, 0, 1
        // y = 1
        Some(Dir::SW), Some(Dir::S), Some(Dir::SE), // x = -1, 0, 1
    ];

    #[inline(always)]
    pub const fn from_vector(vector: IVec2) -> Option<Self> {
        // .signum() reduces (-1..=1) range and allows non-unit vectors (e.g. (10, -10) -> NE)
        let x = (vector.x.signum() + 1) as usize;
        let y = (vector.y.signum() + 1) as usize;

        Self::FROM_VEC_LUT[y * 3 + x]
    }

    // #[inline(always)]
    // pub fn from_vector_unchecked(vector: IVec2) -> Self {
    //     // .signum() reduces (-1..=1) range and allows non-unit vectors (e.g. (10, -10) -> NE)
    //     let x = (vector.x.signum() + 1) as usize;
    //     let y = (vector.y.signum() + 1) as usize;

    //     let idx = y * 3 + x;

    //     match Self::FROM_VEC_LUT[idx] {
    //         Some(dir) => dir,
    //         None => panic!("Vector {vector} has no valid direction"),
    //     }
    // }

    /// Returns the exact opposite direction (180 degrees).
    #[inline(always)]
    pub const fn opposite(self) -> Self {
        // Bitwise XOR by 4 flips the direction without addition, modulo, or bounds checks
        // In binary (000 to 111), toggling the 3rd bit (the 4s place) flips any direction to its exact opposite
        unsafe { std::mem::transmute(self as u8 ^ 4) }
    }

    const ANGLE_DIFF_LUT: [DirAngle; 8] = [
        DirAngle::Deg0,   // diff = 0
        DirAngle::Deg45,  // diff = 1
        DirAngle::Deg90,  // diff = 2
        DirAngle::Deg135, // diff = 3
        DirAngle::Deg180, // diff = 4
        DirAngle::Deg135, // diff = 5 (wraps to 3 steps)
        DirAngle::Deg90,  // diff = 6 (wraps to 2 steps)
        DirAngle::Deg45,  // diff = 7 (wraps to 1 step)
    ];

    /// Calculates the shortest angular difference between two directions.
    #[inline(always)]
    pub const fn angle_diff(self, other: Self) -> DirAngle {
        let diff = (self as u8).abs_diff(other as u8) as usize;

        Self::ANGLE_DIFF_LUT[diff]
    }

    #[inline(always)]
    pub const fn is_diagonal(self) -> bool {
        (self as u8) & 1 == 1
    }

    /// General turn function: rotates by N 45-degree steps.
    /// Positive steps turn right (clockwise), negative steps turn left (anti-clockwise).
    ///
    /// Examples:
    /// - `1`  = 45° Right
    /// - `2`  = 90° Right
    /// - `-1` = 45° Left
    /// - `-2` = 90° Left
    #[inline(always)]
    pub const fn turn(self, steps: i8) -> Self {
        // `steps as u8` uses two's complement wrapping under the hood.
        // Bitwise `& 7` automatically wraps both positive and negative values into 0..=7.
        let raw = (self as u8).wrapping_add(steps as u8) & 7;

        unsafe { std::mem::transmute(raw) }
    }

    #[inline(always)]
    pub const fn turn_right_45(self) -> Self {
        self.turn(1)
    }

    #[inline(always)]
    pub const fn turn_right_90(self) -> Self {
        self.turn(2)
    }

    #[inline(always)]
    pub const fn turn_right_135(self) -> Self {
        self.turn(3)
    }

    #[inline(always)]
    pub const fn turn_left_45(self) -> Self {
        self.turn(-1)
    }

    #[inline(always)]
    pub const fn turn_left_90(self) -> Self {
        self.turn(-2)
    }

    #[inline(always)]
    pub const fn turn_left_135(self) -> Self {
        self.turn(-3)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DirAngle {
    Deg0 = 0,
    Deg45 = 1,
    Deg90 = 2,
    Deg135 = 3,
    Deg180 = 4,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Vertical,
    Horizontal,
    DiagNE,
    DiagNW,
}

impl From<Dir> for Orientation {
    fn from(dir: Dir) -> Self {
        match dir {
            N | S => Self::Vertical,
            E | W => Self::Horizontal,
            NE | SW => Self::DiagNE,
            NW | SE => Self::DiagNW,
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Orientation::Vertical => "Vertical",
            Orientation::Horizontal => "Horizontal",
            Orientation::DiagNE => "DiagNE",
            Orientation::DiagNW => "DiagNW",
        };

        write!(f, "{s}")
    }
}

impl Orientation {
    #[must_use]
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Orientation::Vertical => Orientation::Horizontal,
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::DiagNE => Orientation::DiagNW,
            Orientation::DiagNW => Orientation::DiagNE,
        }
    }

    #[must_use]
    #[inline]
    pub fn random(rng: &mut Rng) -> Self {
        match rng.gen_range(0..4) {
            0 => Self::Vertical,
            1 => Self::Horizontal,
            2 => Self::DiagNE,
            3 => Self::DiagNW,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    #[must_use]
    #[inline]
    pub const fn get_directions(&self) -> [Dir; 2] {
        match self {
            Orientation::Vertical => [N, S],
            Orientation::Horizontal => [E, W],
            Orientation::DiagNE => [NE, SW],
            Orientation::DiagNW => [NW, SE],
        }
    }
}
