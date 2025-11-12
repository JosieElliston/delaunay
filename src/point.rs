use std::ops::{Add, AddAssign, Sub, SubAssign};

pub(crate) type Coord = f64;
pub(crate) type PointId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Point {
    x: Coord,
    y: Coord,
}
impl Point {
    pub(crate) const fn new(x: Coord, y: Coord) -> Self {
        assert!(!x.is_nan() && !y.is_nan());
        Self { x, y }
    }

    pub(crate) const fn x(&self) -> Coord {
        self.x
    }

    pub(crate) const fn y(&self) -> Coord {
        self.y
    }
}

impl From<Point> for (Coord, Coord) {
    fn from(p: Point) -> Self {
        (p.x(), p.y())
    }
}
impl From<(Coord, Coord)> for Point {
    fn from(t: (Coord, Coord)) -> Self {
        Point::new(t.0, t.1)
    }
}
impl From<(f32, f32)> for Point {
    fn from(t: (f32, f32)) -> Self {
        Point::new(t.0 as Coord, t.1 as Coord)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Add<Point> for Point {
    type Output = Point;

    fn add(mut self, other: Point) -> Point {
        self += other;
        self
    }
}
impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl Sub<Point> for Point {
    type Output = Point;

    fn sub(mut self, other: Point) -> Point {
        self -= other;
        self
    }
}
