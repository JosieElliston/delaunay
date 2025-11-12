pub(crate) type Coord = f64;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Point {
    x: Coord,
    y: Coord,
}
impl Point {
    fn new(x: Coord, y: Coord) -> Self {
        assert!(!x.is_nan() && !y.is_nan());
        Self { x, y }
    }

    fn x(&self) -> Coord {
        self.x
    }

    fn y(&self) -> Coord {
        self.y
    }
}
