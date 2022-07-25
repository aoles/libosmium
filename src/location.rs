use std::fmt::{Display, Formatter};

const UNDEFINED_COORDINATE: i32 = i32::MAX;
const PRECISION: i32 = 10000000;

/// A gps coordinate in angles of latitude and longitude
///
/// The actual data is stored in `x` and `y` as integers which are `1/PRECISION`-th of a degree.
///
/// [Libosmium's cpp reference](https://docs.osmcode.org/libosmium/latest/classosmium_1_1Location.html)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Location {
    pub fn is_defined(&self) -> bool {
        self.x != UNDEFINED_COORDINATE || self.y != UNDEFINED_COORDINATE
    }

    pub fn is_undefined(&self) -> bool {
        self.x == UNDEFINED_COORDINATE && self.y == UNDEFINED_COORDINATE
    }

    pub fn is_valid(&self) -> bool {
        self.x >= -180 * PRECISION
            && self.x <= 180 * PRECISION
            && self.y >= -90 * PRECISION
            && self.y <= 90 * PRECISION
    }

    pub fn lon(&self) -> f64 {
        self.x as f64 / PRECISION as f64
    }

    pub fn lat(&self) -> f64 {
        self.y as f64 / PRECISION as f64
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (mut x1, mut x2) = (self.x / PRECISION, self.x % PRECISION);
        let (mut y1, mut y2) = (self.y / PRECISION, self.y % PRECISION);
        x2 = x2.abs(); y2 = y2.abs();
        let lat = if self.y > 0 { "N" } else { y1 = -y1; "S" };
        let lon = if self.x > 0 { "E" } else { x1 = -x1; "W" };
        write!(f, "{}.{}° {} {}.{}° {}", y1, y2, lat, x1, x2, lon)
    }
}